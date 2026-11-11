# Windows API fallback

Windows DLL dispatch uses full or partial API handlers where available. Banzai provides optional fallback for calls without such handlers. The rules below describe the common x86/x64 fallback; direct syscalls, kernel emulation, and special top-level routes have separate handling.

## Banzai

Enable banzai with `--banzai`, `emu.enable_banzai()` in Rust, or `emu.enable_banzai_mode()` in Python. `Config::skip_unimplemented` controls API skipping.

Unhandled calls may return zero without emulating side effects or writing output buffers:

| Architecture | Requirements and stack cleanup |
| --- | --- |
| x86 | A nonnegative cleanup size from banzai metadata is required; that many DWORDs are removed from the stack. An unknown size stops emulation. |
| x64 | No size metadata is required. Stack arguments and shadow space remain owned by the caller. |

The gateway has already consumed the return address. With skipping disabled, the common fallback logs the unhandled call and stops. Banzai does not override full or partial handlers.

## NT APIs

The ntdll dispatchers normalize `Zw` names to `Nt` before looking for handlers. Known unsupported NTSTATUS calls use the shared NT signature catalog to return `STATUS_NOT_IMPLEMENTED` (`0xc0000002`), with x86 argument cleanup. This partial handling works regardless of banzai mode.

The catalog contains complete `Nt` names and x86 stack sizes in DWORDs. Only verified NTSTATUS-returning signatures belong there. Unsupported Nt/Zw calls outside the catalog stop; they never fall back to banzai metadata. Existing handlers retain priority. Adding a catalog entry does not add a DLL export or implement API side effects.

## Banzai metadata

`banzai.csv` is a headerless collection of exact, case-sensitive API names and integer x86 callee cleanup sizes:

```csv
GetSystemTimeAsFileTime,1
GetTickCount,0
```

Sizes are in **DWORDs**, excluding the return address, rather than C parameter counts. A 64-bit value passed by value occupies two DWORDs; an x86 pointer occupies one. Caller-cleanup functions need zero. Incorrect metadata can move ESP to the wrong address.

Malformed records are logged and skipped; later valid duplicates replace earlier entries. Nt/Zw rows remain in the collection but are not used by dispatch.

Windows initialization loads `<maps_folder>/banzai.csv` only for x86 with skipping enabled. Enable banzai before loading the binary if CSV metadata is needed; enabling it afterward does not reload the file.

Use `banzai_add(name, nparams)` in Rust or Python to add or replace a cleanup size at runtime. `nparams` is the DWORD size described above. Apply overrides after loading the binary so CSV loading cannot replace them.

## Diagnostics

Fallback returns log the API name, address, instruction pointer, cleanup size, and result. Stops log the reason and serialize emulator state if `dump_on_exit` and `dump_filename` are configured.

`--strict-api` / `Config::stop_on_unimplemented` adds an x64 check for calls still reported as unhandled by a gateway. It does not override a defined fallback return.

## Adding handlers

Keep A/W variants in the same module and connect each implementation through a single module owner. Run `make api-modules` after adding or moving handlers. CI compares Windows API source files with the dependencies of the current library and test compilation, and rejects unconnected Rust files.
