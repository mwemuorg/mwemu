// SPDX-License-Identifier: GPL-2.0
/*
 * tlm — a deliberately vulnerable telemetry channel driver, used as the
 * reference target for mwemu's kernel-mode emulation.
 *
 * It is written the way a real small driver is written: a refcounted object
 * kept in a kmem_cache, a linked list of live objects behind a mutex, an ioctl
 * surface, per-object operation vectors, and a payload buffer allocated
 * separately from the object that owns it.
 *
 * The bug is a use-after-free, and it is the interesting kind: nothing here
 * frees an object and then obviously touches it two lines later. The driver
 * keeps a one-entry "hot channel" cache so that a stream of writes to the same
 * channel skips the list walk. The cache deliberately holds no reference —
 * see the comment on tlm_device::fast — and it is invalidated when the file
 * handle is closed and when the module unloads. What its author missed is the
 * third way a channel can go away: TLM_IOC_DESTROY drops the list's reference
 * while the file handle stays open, so the cache is left pointing at a freed
 * object and the next write to that id runs the whole write path against it.
 *
 * Reachability: open the device, create a channel, write to it once (which
 * populates the cache), destroy it, write to it again.
 */

#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>
#include <linux/fs.h>
#include <linux/slab.h>
#include <linux/uaccess.h>
#include <linux/miscdevice.h>
#include <linux/mutex.h>
#include <linux/list.h>
#include <linux/refcount.h>

#define TLM_NAME_LEN	24
#define TLM_MAGIC	0x544c4d30	/* "TLM0" */
#define TLM_MAX_BUF	4096

#define TLM_IOC_CREATE	0x1001
#define TLM_IOC_WRITE	0x1002
#define TLM_IOC_DESTROY	0x1003
#define TLM_IOC_STAT	0x1004

/* Encoding selected at channel creation time. */
#define TLM_ENC_RAW	0
#define TLM_ENC_DELTA	1

struct tlm_create_req {
	char	name[TLM_NAME_LEN];
	__u32	buf_len;
	__u32	encoding;
	__u32	id_out;
};

struct tlm_write_req {
	__u32	id;
	__u32	len;
	__u64	data;
};

struct tlm_id_req {
	__u32	id;
};

struct tlm_stat_req {
	__u32	id;
	__u32	used;
	__u32	capacity;
};

struct tlm_channel;

struct tlm_ops {
	const char *name;
	int (*encode)(struct tlm_channel *ch, const u8 *src, u32 len);
	void (*reset)(struct tlm_channel *ch);
};

struct tlm_channel {
	u32			magic;
	u32			id;
	refcount_t		refs;
	u32			encoding;
	char			name[TLM_NAME_LEN];
	const struct tlm_ops	*ops;
	u8			*buf;
	u32			buf_len;
	u32			used;
	u8			last;
	struct list_head	node;
};

struct tlm_device {
	struct mutex		lock;
	struct list_head	channels;
	struct kmem_cache	*cache;
	u32			next_id;
	unsigned long		writes;

	/*
	 * One-entry hot-channel cache. It holds no reference on purpose: a
	 * cache that pinned channels would keep them alive past their last
	 * user, so the rule is that whoever removes a channel from the list
	 * also clears the cache.
	 */
	struct tlm_channel	*fast;
	u32			fast_id;
};

static struct tlm_device tlm_dev;

/* ---------------------------------------------------------------- encoders */

static int tlm_encode_raw(struct tlm_channel *ch, const u8 *src, u32 len)
{
	if (len > ch->buf_len - ch->used)
		return -ENOSPC;

	memcpy(ch->buf + ch->used, src, len);
	ch->used += len;
	return len;
}

static int tlm_encode_delta(struct tlm_channel *ch, const u8 *src, u32 len)
{
	u32 i;

	if (len > ch->buf_len - ch->used)
		return -ENOSPC;

	for (i = 0; i < len; i++) {
		ch->buf[ch->used + i] = src[i] - ch->last;
		ch->last = src[i];
	}
	ch->used += len;
	return len;
}

static void tlm_reset_common(struct tlm_channel *ch)
{
	ch->used = 0;
	ch->last = 0;
}

static const struct tlm_ops tlm_ops_raw = {
	.name	= "raw",
	.encode	= tlm_encode_raw,
	.reset	= tlm_reset_common,
};

static const struct tlm_ops tlm_ops_delta = {
	.name	= "delta",
	.encode	= tlm_encode_delta,
	.reset	= tlm_reset_common,
};

/* ------------------------------------------------------------- object life */

static void tlm_channel_release(struct tlm_channel *ch)
{
	pr_info("tlm: releasing channel %u (%s)\n", ch->id, ch->name);
	kfree(ch->buf);
	kmem_cache_free(tlm_dev.cache, ch);
}

static void tlm_channel_put(struct tlm_channel *ch)
{
	if (refcount_dec_and_test(&ch->refs))
		tlm_channel_release(ch);
}

/* Caller must hold dev->lock. Returns a borrowed pointer. */
static struct tlm_channel *tlm_lookup_locked(struct tlm_device *dev, u32 id)
{
	struct tlm_channel *ch;

	list_for_each_entry(ch, &dev->channels, node) {
		if (ch->id == id)
			return ch;
	}
	return NULL;
}

/* Returns a channel with an extra reference taken, or NULL. */
static struct tlm_channel *tlm_channel_get(struct tlm_device *dev, u32 id)
{
	struct tlm_channel *ch;

	mutex_lock(&dev->lock);
	ch = tlm_lookup_locked(dev, id);
	if (ch)
		refcount_inc(&ch->refs);
	mutex_unlock(&dev->lock);

	return ch;
}

/* --------------------------------------------------------------- ioctl ops */

static int tlm_do_create(struct tlm_device *dev, struct tlm_create_req *req)
{
	struct tlm_channel *ch;

	if (req->buf_len == 0 || req->buf_len > TLM_MAX_BUF)
		return -EINVAL;
	if (req->encoding > TLM_ENC_DELTA)
		return -EINVAL;

	ch = kmem_cache_alloc(dev->cache, GFP_KERNEL);
	if (!ch)
		return -ENOMEM;

	memset(ch, 0, sizeof(*ch));
	ch->buf = kzalloc(req->buf_len, GFP_KERNEL);
	if (!ch->buf) {
		kmem_cache_free(dev->cache, ch);
		return -ENOMEM;
	}

	ch->magic = TLM_MAGIC;
	ch->buf_len = req->buf_len;
	ch->encoding = req->encoding;
	ch->ops = req->encoding == TLM_ENC_DELTA ? &tlm_ops_delta : &tlm_ops_raw;
	req->name[TLM_NAME_LEN - 1] = '\0';
	strscpy(ch->name, req->name, TLM_NAME_LEN);

	/* One reference for the list; the caller gets no reference back. */
	refcount_set(&ch->refs, 1);

	mutex_lock(&dev->lock);
	ch->id = ++dev->next_id;
	list_add_tail(&ch->node, &dev->channels);
	mutex_unlock(&dev->lock);

	req->id_out = ch->id;
	pr_info("tlm: created channel %u (%s, %s, %u bytes)\n",
		ch->id, ch->name, ch->ops->name, ch->buf_len);
	return 0;
}

static int tlm_do_write(struct tlm_device *dev, struct tlm_write_req *req)
{
	struct tlm_channel *ch;
	bool borrowed = true;
	u8 *kbuf;
	int ret;

	if (req->len == 0 || req->len > TLM_MAX_BUF)
		return -EINVAL;

	if (dev->fast && dev->fast_id == req->id) {
		/* Hot path: same channel as last time, skip the list walk. */
		ch = dev->fast;
	} else {
		ch = tlm_channel_get(dev, req->id);
		if (!ch)
			return -ENOENT;
		if (ch->magic != TLM_MAGIC) {
			tlm_channel_put(ch);
			return -EBADF;
		}
		borrowed = false;
		dev->fast = ch;
		dev->fast_id = ch->id;
	}

	kbuf = kmalloc(req->len, GFP_KERNEL);
	if (!kbuf) {
		ret = -ENOMEM;
		goto out;
	}

	if (copy_from_user(kbuf, (void __user *)(uintptr_t)req->data, req->len)) {
		kfree(kbuf);
		ret = -EFAULT;
		goto out;
	}

	ret = ch->ops->encode(ch, kbuf, req->len);
	kfree(kbuf);
	dev->writes++;

out:
	if (!borrowed)
		tlm_channel_put(ch);
	return ret;
}

static int tlm_do_destroy(struct tlm_device *dev, u32 id)
{
	struct tlm_channel *ch;

	mutex_lock(&dev->lock);
	ch = tlm_lookup_locked(dev, id);
	if (!ch) {
		mutex_unlock(&dev->lock);
		return -ENOENT;
	}
	list_del_init(&ch->node);
	mutex_unlock(&dev->lock);

	/* Drop the reference the channel list was holding. */
	tlm_channel_put(ch);
	return 0;
}

static int tlm_do_stat(struct tlm_device *dev, struct tlm_stat_req *req)
{
	struct tlm_channel *ch;

	ch = tlm_channel_get(dev, req->id);
	if (!ch)
		return -ENOENT;

	req->used = ch->used;
	req->capacity = ch->buf_len;
	tlm_channel_put(ch);
	return 0;
}

/* ------------------------------------------------------------ file ops */

static long tlm_ioctl(struct file *filp, unsigned int cmd, unsigned long arg)
{
	void __user *argp = (void __user *)arg;
	struct tlm_device *dev = &tlm_dev;
	int ret;

	switch (cmd) {
	case TLM_IOC_CREATE: {
		struct tlm_create_req req;

		if (copy_from_user(&req, argp, sizeof(req)))
			return -EFAULT;
		ret = tlm_do_create(dev, &req);
		if (ret == 0 && copy_to_user(argp, &req, sizeof(req)))
			return -EFAULT;
		return ret;
	}
	case TLM_IOC_WRITE: {
		struct tlm_write_req req;

		if (copy_from_user(&req, argp, sizeof(req)))
			return -EFAULT;
		return tlm_do_write(dev, &req);
	}
	case TLM_IOC_DESTROY: {
		struct tlm_id_req req;

		if (copy_from_user(&req, argp, sizeof(req)))
			return -EFAULT;
		return tlm_do_destroy(dev, req.id);
	}
	case TLM_IOC_STAT: {
		struct tlm_stat_req req;

		if (copy_from_user(&req, argp, sizeof(req)))
			return -EFAULT;
		ret = tlm_do_stat(dev, &req);
		if (ret == 0 && copy_to_user(argp, &req, sizeof(req)))
			return -EFAULT;
		return ret;
	}
	default:
		return -ENOTTY;
	}
}

static int tlm_open(struct inode *inode, struct file *filp)
{
	return 0;
}

static int tlm_close(struct inode *inode, struct file *filp)
{
	/* Closing the handle drops the hot-channel cache. */
	tlm_dev.fast = NULL;
	tlm_dev.fast_id = 0;
	return 0;
}

static const struct file_operations tlm_fops = {
	.owner		= THIS_MODULE,
	.open		= tlm_open,
	.release	= tlm_close,
	.unlocked_ioctl	= tlm_ioctl,
};

static struct miscdevice tlm_misc = {
	.minor	= MISC_DYNAMIC_MINOR,
	.name	= "tlm",
	.fops	= &tlm_fops,
};

/* ---------------------------------------------------------------- module */

static int __init tlm_init(void)
{
	int ret;

	mutex_init(&tlm_dev.lock);
	INIT_LIST_HEAD(&tlm_dev.channels);
	tlm_dev.next_id = 0;
	tlm_dev.fast = NULL;
	tlm_dev.fast_id = 0;

	tlm_dev.cache = kmem_cache_create("tlm_channel",
					  sizeof(struct tlm_channel),
					  0, SLAB_HWCACHE_ALIGN, NULL);
	if (!tlm_dev.cache)
		return -ENOMEM;

	ret = misc_register(&tlm_misc);
	if (ret) {
		kmem_cache_destroy(tlm_dev.cache);
		return ret;
	}

	pr_info("tlm: telemetry driver loaded\n");
	return 0;
}

static void __exit tlm_exit(void)
{
	struct tlm_channel *ch, *tmp;

	misc_deregister(&tlm_misc);

	tlm_dev.fast = NULL;
	tlm_dev.fast_id = 0;

	mutex_lock(&tlm_dev.lock);
	list_for_each_entry_safe(ch, tmp, &tlm_dev.channels, node) {
		list_del_init(&ch->node);
		tlm_channel_put(ch);
	}
	mutex_unlock(&tlm_dev.lock);

	kmem_cache_destroy(tlm_dev.cache);
	pr_info("tlm: telemetry driver unloaded\n");
}

module_init(tlm_init);
module_exit(tlm_exit);

MODULE_LICENSE("GPL");
MODULE_AUTHOR("mwemu");
MODULE_DESCRIPTION("Deliberately vulnerable telemetry channel driver (mwemu test target)");
MODULE_VERSION("1.0");
