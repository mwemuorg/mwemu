# cmwemu
[![Rust CI](https://github.com/mwemuorg/cmwemu/actions/workflows/ci.yaml/badge.svg)](https://github.com/mwemuorg/cmwemu/actions/workflows/ci.yaml)

C ABI bindings for [`libmwemu`](../libmwemu) — the x86/x64/arm64 emulator. This
is the C counterpart of `pymwemu`: a thin `extern "C"` surface over the
`libmwemu::emu::Emu` object, mirroring the same API.

## Build

```sh
cargo build          # or --release
```

This produces, under `target/<profile>/`:

- `libcmwemu.so` / `.dylib` / `.dll` — dynamic library (`cdylib`)
- `libcmwemu.a` — static library (`staticlib`)

and regenerates the C header at [`include/mwemu.h`](include/mwemu.h) via
`cbindgen` (the header is committed, so C consumers don't need a Rust build).

## Usage

```c
#include "mwemu.h"

struct MwemuEmu *emu = mwemu_init64();      /* or _init32 / _init_aarch64 */
mwemu_load_code_bytes(emu, code, code_len);
mwemu_step(emu);

uint64_t rax = 0;
mwemu_get_reg(emu, "rax", &rax);

mwemu_free_emu(emu);                        /* destructor */
```

See [`examples/demo.c`](examples/demo.c) and its `Makefile`:

```sh
cd examples && make run        # dynamic link
cd examples && make static     # self-contained binary
```

## Conventions

- **Handle.** The emulator is an opaque `MwemuEmu *` from `mwemu_init*` /
  `mwemu_load*`; destroy it with `mwemu_free_emu`. Every function takes the
  handle first; a NULL handle is a safe no-op that records an error.
- **Errors.** Fallible functions return `int32_t` (`1` = ok, `0` = error) and
  write any result through an out-pointer. Retrieve the message with
  `mwemu_last_error()` (thread-local, valid until the next call on that thread).
- **Ownership.** `char *` results are freed with `mwemu_free_string`; byte
  buffers with `mwemu_free_buffer`; `uint64_t` arrays with
  `mwemu_free_u64_buffer`.
- **Strings / blobs.** Inputs are NUL-terminated UTF-8 `const char *` and
  `(const uint8_t *, size_t)`. 128-bit values use a `lo`/`hi` `uint64_t` pair.
- **Permissions.** `MWEMU_PERM_READ | _WRITE | _EXECUTE`, or `MWEMU_PERM_RWX`.
