# tlm — a deliberately vulnerable telemetry driver

The reference target for mwemu's kernel-mode emulation. It is a small but
realistic character driver: refcounted objects living in their own
`kmem_cache`, a mutex-protected list of live channels, per-object operation
vectors, a payload buffer allocated separately from the object that owns it,
and an ioctl surface on top.

> **Do not `insmod` this.** It is written to be linked and run *inside* mwemu.

## The bug

Not the textbook "free it, then read it two lines later". The driver keeps a
one-entry **hot-channel cache** so a stream of writes to the same channel skips
the list walk:

```c
struct tlm_device {
	...
	/*
	 * One-entry hot-channel cache. It holds no reference on purpose: a
	 * cache that pinned channels would keep them alive past their last
	 * user, so the rule is that whoever removes a channel from the list
	 * also clears the cache.
	 */
	struct tlm_channel	*fast;
	u32			fast_id;
};
```

The rule in that comment is followed in `tlm_close()` (the file handle went
away) and in `tlm_exit()` (the module is unloading). What its author missed is
the third way a channel can go away: `TLM_IOC_DESTROY` drops the list's
reference — freeing the object — while the file handle stays open. The cache is
left pointing at freed memory, and the next write to that id takes the hot path
straight through it:

```c
	if (dev->fast && dev->fast_id == req->id) {
		/* Hot path: same channel as last time, skip the list walk. */
		ch = dev->fast;
	} else {
		...
	}
	...
	ret = ch->ops->encode(ch, kbuf, req->len);
```

The hot path also skips the `magic` validation the slow path does — plausible
for a fast path, and what lets the freed object get all the way to an indirect
call through `ch->ops`.

**Trigger:** create a channel, write to it once (which populates the cache),
destroy it, write to it again.

## Building

```sh
make driver            # from the repo root -> test/linux_uaf_driver.ko
make                   # here, against the running kernel's headers
make KDIR=/path/to/src # against another kernel tree
```

Needs the running kernel's headers (`/lib/modules/$(uname -r)/build`). The
kernel-emulation tests skip themselves when the artefact is absent.

## Reproducing the finding

```sh
make driver
cargo test -p libmwemu tests::kernel -- --nocapture
```

or from the CLI, which links the module and runs its init the way `insmod`
would:

```sh
cargo run -p mwemu -- -f test/linux_uaf_driver.ko -6 -v
```

Driving the ioctl surface (and therefore reaching the bug) needs argument
structs in guest memory, so it is done from Rust (`Emu::call_module_symbol`),
from the tests above, or over MCP with `mwemu_kernel_call`. What mwemu reports:

```
BUG: KMWEMU: use-after-free (read) in tlm_channel of size 8 at addr 0xffff888000001128
  faulting instruction at 0xffffffffc00007fb (step 478)
  object 0xffff888000001100..0xffff888000001160 (requested 88 bytes, bucket 96), offset 40
  allocated by kmem_cache_alloc_noprof at 0xffffffffc000056c (step 86)
  freed by kmem_cache_free at 0xffffffffc00004e9 (step 407)

BUG: KMWEMU: use-after-free (poisoned pointer dereference) at addr 0x6b6b6b6b6b6b6b73
  faulting instruction at 0xffffffffc000080e (step 483)
```

The first is the load of `ch->ops` out of the quarantined object; the second is
the dereference of the pointer that load produced — `0x6b6b…` is SLUB's free
poison, so its provenance is not a guess.

## ioctl surface

| command | value | argument |
| --- | --- | --- |
| `TLM_IOC_CREATE` | `0x1001` | `struct tlm_create_req { char name[24]; u32 buf_len; u32 encoding; u32 id_out; }` |
| `TLM_IOC_WRITE` | `0x1002` | `struct tlm_write_req { u32 id; u32 len; u64 data; }` |
| `TLM_IOC_DESTROY` | `0x1003` | `struct tlm_id_req { u32 id; }` |
| `TLM_IOC_STAT` | `0x1004` | `struct tlm_stat_req { u32 id; u32 used; u32 capacity; }` |

`encoding` is `0` (raw) or `1` (delta).
