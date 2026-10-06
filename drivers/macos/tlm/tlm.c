/*
 * tlm — the macOS counterpart of drivers/linux/tlm, a deliberately vulnerable
 * telemetry-channel kext used as the reference target for mwemu's kext
 * emulation. It is a plain BSD (kmod) kext — no IOKit matching — written the
 * way a small driver is: refcounted-ish channel objects allocated with
 * IOMalloc, per-channel operation vectors, a payload buffer allocated
 * separately from the object that owns it, and an ioctl-style dispatch.
 *
 * > Do not kextload this. It is written to be linked and run *inside* mwemu,
 * > never on a real kernel, and it is deliberately buggy.
 *
 * The bug is identical in spirit to the Linux fixture: a one-entry "hot
 * channel" cache that holds no reference. It is cleared when the kext stops,
 * but TLM_IOC_DESTROY frees a channel while leaving the cache pointing at it,
 * so the next write to that id takes the hot path straight through freed
 * memory — reading the stale object and copying into its freed buffer.
 *
 * Trigger: create a channel, write to it once (which populates the cache),
 * destroy it, write to it again.
 */

#include <mach/mach_types.h>
#include <libkern/libkern.h>
#include <IOKit/IOLib.h>
#include <string.h>

#define TLM_NAME_LEN	24
#define TLM_MAGIC	0x544c4d30	/* "TLM0" */
#define TLM_MAX_CHAN	8

#define TLM_IOC_CREATE	0x1001
#define TLM_IOC_WRITE	0x1002
#define TLM_IOC_DESTROY	0x1003

#define TLM_ENC_RAW	0
#define TLM_ENC_DELTA	1

struct tlm_create_req {
	char		name[TLM_NAME_LEN];
	uint32_t	buf_len;
	uint32_t	encoding;
	uint32_t	id_out;
};

struct tlm_write_req {
	uint32_t	id;
	uint32_t	len;
	uint64_t	data;
};

struct tlm_id_req {
	uint32_t	id;
};

struct tlm_channel;

struct tlm_ops {
	const char	*name;
	int		(*encode)(struct tlm_channel *ch, const uint8_t *src, uint32_t len);
};

struct tlm_channel {
	uint32_t		magic;
	uint32_t		id;
	uint32_t		encoding;
	char			name[TLM_NAME_LEN];
	const struct tlm_ops	*ops;
	uint8_t			*buf;
	uint32_t		buf_len;
	uint32_t		used;
	uint8_t			last;
};

struct tlm_device {
	struct tlm_channel	*chans[TLM_MAX_CHAN];
	uint32_t		n;
	uint32_t		next_id;

	/*
	 * One-entry hot-channel cache, holding no reference on purpose: a cache
	 * that pinned channels would keep them alive past their last user, so
	 * whoever removes a channel is supposed to clear the cache. tlm_stop()
	 * does; TLM_IOC_DESTROY forgets to — that is the bug.
	 */
	struct tlm_channel	*fast;
	uint32_t		fast_id;
};

static struct tlm_device tlm_dev;

/* ---------------------------------------------------------------- encoders */

static int tlm_encode_raw(struct tlm_channel *ch, const uint8_t *src, uint32_t len)
{
	if (len > ch->buf_len - ch->used)
		return -1;
	memcpy(ch->buf + ch->used, src, len);
	ch->used += len;
	return (int)len;
}

static int tlm_encode_delta(struct tlm_channel *ch, const uint8_t *src, uint32_t len)
{
	uint32_t i;

	if (len > ch->buf_len - ch->used)
		return -1;
	for (i = 0; i < len; i++) {
		ch->buf[ch->used + i] = src[i] - ch->last;
		ch->last = src[i];
	}
	ch->used += len;
	return (int)len;
}

static const struct tlm_ops tlm_ops_raw = { "raw", tlm_encode_raw };
static const struct tlm_ops tlm_ops_delta = { "delta", tlm_encode_delta };

/* ---------------------------------------------------------------- helpers */

static struct tlm_channel *tlm_find(uint32_t id)
{
	uint32_t i;

	for (i = 0; i < tlm_dev.n; i++)
		if (tlm_dev.chans[i]->id == id)
			return tlm_dev.chans[i];
	return NULL;
}

static void tlm_free_channel(struct tlm_channel *ch)
{
	IOFree(ch->buf, ch->buf_len);
	IOFree(ch, sizeof(*ch));
}

/* ---------------------------------------------------------------- ioctl */

static int tlm_create(struct tlm_create_req *req)
{
	struct tlm_channel *ch;

	if (tlm_dev.n >= TLM_MAX_CHAN)
		return -1;

	ch = (struct tlm_channel *)IOMalloc(sizeof(*ch));
	if (!ch)
		return -1;
	ch->buf = (uint8_t *)IOMalloc(req->buf_len);
	if (!ch->buf) {
		IOFree(ch, sizeof(*ch));
		return -1;
	}

	ch->magic = TLM_MAGIC;
	ch->id = ++tlm_dev.next_id;
	ch->encoding = req->encoding;
	memcpy(ch->name, req->name, TLM_NAME_LEN);
	ch->ops = (req->encoding == TLM_ENC_DELTA) ? &tlm_ops_delta : &tlm_ops_raw;
	ch->buf_len = req->buf_len;
	ch->used = 0;
	ch->last = 0;

	tlm_dev.chans[tlm_dev.n++] = ch;
	req->id_out = ch->id;
	IOLog("tlm: created channel %u (%s)\n", ch->id, ch->ops->name);
	return 0;
}

static int tlm_write(struct tlm_write_req *req)
{
	struct tlm_channel *ch;

	if (tlm_dev.fast && tlm_dev.fast_id == req->id) {
		/* Hot path: same channel as last time, skip the list walk and
		 * the magic check. This is where a stale cache entry is used. */
		ch = tlm_dev.fast;
	} else {
		ch = tlm_find(req->id);
		if (!ch || ch->magic != TLM_MAGIC)
			return -1;
		tlm_dev.fast = ch;
		tlm_dev.fast_id = req->id;
	}

	return ch->ops->encode(ch, (const uint8_t *)req->data, req->len);
}

static int tlm_destroy(struct tlm_id_req *req)
{
	uint32_t i;

	for (i = 0; i < tlm_dev.n; i++) {
		struct tlm_channel *ch = tlm_dev.chans[i];
		if (ch->id != req->id)
			continue;
		/* BUG: the list's reference is dropped and the object freed,
		 * but tlm_dev.fast is left pointing at it. */
		tlm_dev.chans[i] = tlm_dev.chans[--tlm_dev.n];
		tlm_free_channel(ch);
		return 0;
	}
	return -1;
}

/*
 * Single dispatch entry the mwemu test calls by symbol, standing in for the
 * character-device ioctl. `arg` is an address in emulated memory the caller
 * has already populated, so it is dereferenced directly (no copyin needed
 * inside the emulator).
 */
int tlm_ioctl(void *dev, uint32_t cmd, uint64_t arg);
int tlm_ioctl(void *dev, uint32_t cmd, uint64_t arg)
{
	(void)dev;
	switch (cmd) {
	case TLM_IOC_CREATE:
		return tlm_create((struct tlm_create_req *)arg);
	case TLM_IOC_WRITE:
		return tlm_write((struct tlm_write_req *)arg);
	case TLM_IOC_DESTROY:
		return tlm_destroy((struct tlm_id_req *)arg);
	default:
		return -1;
	}
}

/* ---------------------------------------------------------------- kmod */

static kern_return_t tlm_start(kmod_info_t *ki, void *d)
{
	(void)ki;
	(void)d;
	tlm_dev.n = 0;
	tlm_dev.next_id = 0;
	tlm_dev.fast = NULL;
	tlm_dev.fast_id = 0;
	IOLog("tlm: telemetry driver loaded\n");
	return KERN_SUCCESS;
}

static kern_return_t tlm_stop(kmod_info_t *ki, void *d)
{
	uint32_t i;

	(void)ki;
	(void)d;
	/* Clearing the cache here is the rule TLM_IOC_DESTROY forgets. */
	tlm_dev.fast = NULL;
	for (i = 0; i < tlm_dev.n; i++)
		tlm_free_channel(tlm_dev.chans[i]);
	tlm_dev.n = 0;
	IOLog("tlm: telemetry driver unloaded\n");
	return KERN_SUCCESS;
}

extern kern_return_t _start(kmod_info_t *ki, void *data);
extern kern_return_t _stop(kmod_info_t *ki, void *data);
KMOD_EXPLICIT_DECL(com.mwemu.tlm, "1.0.0", _start, _stop)
__private_extern__ kmod_start_func_t *_realmain = tlm_start;
__private_extern__ kmod_stop_func_t *_antimain = tlm_stop;
__private_extern__ int _kext_apple_cc = __APPLE_CC__;
