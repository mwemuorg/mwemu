# API emulation

mwemu can intercept library calls, execute mapped library code, or dispatch direct syscalls and kernel routines. These routes share some frame mechanics, but retain distinct calling conventions and policies.

| Surface | Current handling |
| --- | --- |
| Windows user-mode library calls | Full or partial handlers first, then the Windows unsupported-call policy below. |
| Linux/macOS library hooks | Platform handlers and their existing unimplemented-call fallback. Checked frame failures stop emulation. |
| Real mapped libraries | Execute guest instructions when selected by the execution mode. A hook may be bypassed. |
| Direct syscalls and kernel routines | Their platform dispatchers and existing policies. Windows API modes do not configure them. |

## Handler ABI

Platform adapters own the calling conventions and use common checked frame access. Windows exposes `WindowsAbi` with `WinApi(convention)` and `WinSyscall` variants. Unix exposes the `UnixApi` adapter. Initialize the required adapter once, for example `let call = WinApi(Stdcall);`. Use its instance methods to read arguments, such as `call.arg_u16(emu, 0)`. Store the result with `call.set_return(emu, value)` and discard callee-owned arguments with `call.cleanup_args(emu, words)`.

For `WinApi`, the function convention is mandatory: `Stdcall`, `Cdecl`, or non-variadic `Thiscall`. Both platforms provide signed/unsigned integer readers from 8 to 64 bits. Windows also provides `arg_f32` and `arg_f64`. Readers return `Result`. The value width does not change the ABI slot stride.

| Entry variant | Use |
| --- | --- |
| `WinApi(convention)` | Windows API hooks after the gateway consumes the return address. x86 indices count DWORD slots, x64 indices count parameter positions. |
| `WinSyscall` | The existing Windows x64 syscall interception point, with its return address still on the stack. Shared semantics receive the adapter explicitly. |
| `UnixApi` | Linux/macOS function hooks using System V AMD64 or the currently supported AAPCS64 register arguments. Not a Unix syscall adapter. |

The common module contains frame access, typed error data, and reusable scalar result storage. It does not dispatch to a platform or expose stdcall cleanup. Platform modules own register and stack placement and select the shared operations they need.

The kernel bridge selects the matching function-hook layout while retaining its own fault policy. Unsupported conventions return an error instead of borrowing another platform's layout.

General Unix ARM64 stack/FP argument classification and direct Unix syscall adapters remain separate work. Integer variadic readers preserve the distinct System V and Darwin layouts.

Windows and Unix handlers each import their platform's `api_try!` macro at a unit-returning boundary. Internal fallible helpers propagate errors with `?`. Decode required arguments before output writes, allocation, cleanup, or callbacks. Diagnostics alone must not determine validity. The kernel bridge retains its existing fallback policy.

Result setters do not complete calls. `cleanup_args(emu, words)` discards callee-owned x86 DWORD slots without reading them. `set_return(emu, value)` only stores the result. Cleanup leaves the stack unchanged for `Cdecl`, native x64/ARM64, and syscall entries. Count stack slots rather than source parameters: an x86 64-bit integer consumes two slots.

`Thiscall` exposes the implicit object at argument index zero (ECX on x86). Explicit x86 stack slots start at index one and cleanup excludes the object. Native x64/ARM64 retain their standard layouts regardless of the source-level convention. Raw register access remains appropriate for CPU contexts and special intrinsic conventions.

## Frame failures and snapshots

An unreadable, overflowing, or unsupported argument layout is an emulator frame failure, distinct from a guest API error result. Windows and Unix hook failures stop `run()` with an error and prevent `step()` from advancing. They do not fabricate a return value, guest exception, `LastError`, or `errno`, or mark the process terminated. Decode arguments before side effects. The stop mechanism does not roll back arbitrary handler work already performed.

Windows retains its context in `emu.api_policy.last_outcome`. Unix stores the failing library, symbol, address, entry PC/SP, thread, and reason in `emu.unix_api.fault`. Nested Unix calls retain the inner failure context. `clear_outcome()` and `clear_fault()` clear their respective diagnostics. They do not repair the frame or arrange a safe restart.

Native snapshots use version 5 to retain Windows policy/outcomes and Unix pending faults. Older published native formats remain readable. Formats without saved policy start with strict Windows policy, and formats without Unix fault state start without a Unix stop.

Existing Banzai settings are preserved. Platform API state is separate from the legacy `Config` layout. Serializing `Config` alone does not retain it.

## Linux and macOS hooks

`UnixApi` covers the function-hook boundary, where the return address has already been consumed or obtained from the link register. Ordinary integer readers cover System V AMD64 register/stack arguments and the currently supported ARM64 register arguments. `integer_vararg` separately selects the existing System V or Darwin integer variadic layout: Linux ARM64 uses remaining argument registers before the stack. Darwin ARM64 uses the stack for these variadic arguments. General ARM64 fixed-argument stack packing, floating-point/aggregate classification, and direct syscall adapters are not implemented by these readers.

The shared `printf` formatter accepts a fallible argument reader. Linux/macOS handlers publish formatted output only after it succeeds. An unavailable variadic argument is not replaced with zero. Errors in API argument decoding use Unix fault state independently of Windows `ApiMode`. Unknown Unix symbols retain their existing dispatcher behavior, including zero-return fallbacks. This does not claim a unified unsupported-call policy or a complete libc model.

Tests of library hooks must select the intended route. In particular, Linux real-libc execution can bypass Rust handlers.

## Windows API policy

Windows DLL dispatch uses existing full or partial handlers first. Unsupported user-mode calls use one policy on x86 and x64, including unresolved imports, CRT fallback, and TLS initialization. Explicit execution of mapped DLL code retains its existing precedence.

### Modes

The default is **`Strict`**. In the CLI, `--banzai` without `--api-mode` or `--strict-api` instead selects `SimulateSuccess` and prints one warning. This does not change the library default.

| Mode | Unsupported call |
| --- | --- |
| `Strict` | Stop before publishing a fallback result or changing output buffers. |
| `SimulateFail` | Return a declared failure when its semantics are known. Otherwise use the generic zero-result fallback. |
| `SimulateSuccess` | Return modeled success when its semantics are known. Otherwise use the same generic zero-result fallback. |

The CLI accepts `--api-mode strict`, `--api-mode simulate-fail`, or `--api-mode simulate-success`. Rust exposes `ApiMode` through `libmwemu` and selects it with `emu.set_api_mode(...)`. Python uses `emu.set_api_mode("strict")`, `"simulate-fail"`, or `"simulate-success"`.

Simulation applies only after normal dispatch reports an unsupported operation. Existing handlers keep their behavior in every mode, including modeled errors and data results such as `IsDebuggerPresent`. This policy does not yet classify every unsupported branch inside partial handlers. Direct syscalls and the driver kernel surface remain separate migration work.

### Declared models

| Surface | `SimulateFail` | `SimulateSuccess` |
| --- | --- | --- |
| `NTSTATUS`-returning Nt/Zw calls in the NT signature catalog | `STATUS_NOT_IMPLEMENTED`, with unchanged output buffers, objects, and `LastError` | `STATUS_SUCCESS`, with unchanged output buffers, objects, scheduler state, and `LastError` |
| `FileTimeToLocalFileTime` and `LocalFileTimeToFileTime` without a handler | `FALSE` and `LastError` = `ERROR_CALL_NOT_IMPLEMENTED`. No output writes | Convert in a fixed UTC environment by copying the `FILETIME` value to a checked output buffer. Preserve `LastError` |
| Other unsupported calls | Generic zero result, subject to ABI checks | The same generic zero result, subject to ABI checks |

Both simulation modes use the entire NT catalog, including user-mode Zw aliases. Failure simulation reports an unsupported operation before executing the API body. Success simulation returns success without implementing that body. Argument validation distinguishes x86 DWORD slots from native parameter positions, and x86 callee cleanup occurs exactly once. x64 retains caller-owned stack space.

Both simulation modes prefer their declared result model and use the same generic fallback when it is absent. Generic fallback writes zero to the integer return register and preserves output buffers, objects, and `LastError`. Zero may mean success, failure, or a null value depending on the API. This best-effort result does not claim an API implementation.

The UTC conversion model rejects unreadable input, unwritable output, and invalid buffer addresses with a controlled stop before publishing success. An explicit model that fails validation does not retry through the generic fallback. UTC conversion does not use the host time zone.

`LastError` is part of each Win32 model, not a global consequence of success or failure. NT result models preserve it. The declared BOOL failure sets it. UTC conversion preserves it on success. Models use the existing emulator error-state helpers. Thread isolation and direct TEB synchronization of that existing state are not implemented by this policy change.

### API policy and Banzai

API mode and Banzai control different mechanisms. Without an explicit policy, `--banzai` selects `SimulateSuccess`. The CLI warns once on stderr that this mode can return success without implementing the operation. An explicit `--api-mode` takes precedence and suppresses the warning. `--strict-api` always forces strict behavior and suppresses it as well, regardless of argument order.

The selected CLI policy also overrides a restored snapshot's policy. Without a policy flag or `--banzai`, restoration keeps the saved policy.

Rust `enable_banzai()`, Python `enable_banzai_mode()`, and `Config::skip_unimplemented` do not choose an API mode. `Config::stop_on_unimplemented` always forces strict behavior.

A subsystem's signature catalog or explicit model supplies ABI information first. Where the subsystem delegates x86 cleanup lookup to generic fallback, Banzai enables CSV or runtime metadata as the last source. That metadata supplies only a cleanup size. It cannot override subsystem ABI checks or define success/failure semantics. On x64 and AArch64 the caller owns the argument area, so generic simulation needs no Banzai cleanup.

| Unsupported call | Banzai disabled | Banzai enabled |
| --- | --- | --- |
| Any API in `Strict` | Stop | Stop |
| Known model in either simulation mode | Apply its failure/success semantics and ABI | Same behavior |
| x86 call delegated to generic cleanup in either simulation mode | Stop with `BanzaiDisabled` | Use a known cleanup size, otherwise stop with `UnknownCleanup` |
| Generic x64/AArch64 call in either simulation mode | Return zero without argument cleanup | Same behavior |

If an unsupported call has only a Banzai cleanup entry, both modes behave alike. `--api-mode simulate-fail --banzai` and `--api-mode simulate-success --banzai` use that cleanup, return zero, and record `SimulatedGeneric`. The modes produce different results only when an explicit result model exists. Banzai's existing instruction-skipping and memory-diagnostic effects remain separate. Selecting an API mode does not change those flags.

No signature catalog yet replaces all cleanup entries supplied by `banzai.csv` or `banzai_add`. PE export indexes identify names and addresses, not argument layouts.

`banzai.csv` contains exact API names and x86 callee cleanup sizes in DWORD slots, excluding the return address. These are not necessarily C parameter counts: a by-value 64-bit argument uses two slots. Malformed rows are skipped and later valid duplicates replace earlier entries. Windows initialization loads the file from the configured maps folder for x86 with Banzai enabled.

Fallback also loads missing defaults on demand, so enabling Banzai after initialization works. This lazy load preserves runtime `banzai_add` overrides and restored entries. Missing or negative counts stop fallback. A known count of zero performs no argument cleanup.

### Stop and diagnostic contract

Windows unsupported-call stops follow the execution contract described above. The entry PC, stack pointer, thread, requested name, library, route, mode, and reason are retained in `emu.api_policy.last_outcome`. Completed simulation records its disposition, return value, and `LastError` action, and increments `simulation_count`. `SimulatedGeneric` identifies a universal stub separately from modeled failure or success.

`UnknownCleanup` means x86 cleanup metadata is unavailable or fails subsystem ABI checks. `BanzaiDisabled` means the optional x86 metadata fallback is disabled. Neither reason describes a missing result model.

A `LastError` action of `None` means preserve, including for stopped calls. It is not the current error value. Logs identify the selected model, including `GenericZero`. Diagnostics are bounded, not an event history.

A pending API stop prevents another run from silently continuing. `clear_outcome()` clears it explicitly but does not repair the unsupported call or arrange a safe restart. For intercepted x86/x64 calls, the retained stack is the entry frame after the guest call placed its return address and before the gateway consumed it. Direct gateway calls retain the frame supplied by their caller.

TLS callbacks use the selected policy throughout loading. A callback that stops or fails to return aborts initialization instead of silently proceeding to the entry point.

## Adding or moving handlers

Keep related variants with their subsystem, connect each implementation through a single module owner, and exercise the actual gateway in tests. Preserve invoked-symbol tracing and distinguish supported behavior from fallback simulation.

For Windows handlers, keep A/W variants in the same module where their contracts agree and run `make api-modules` after adding or moving sources. That compiler-dependency check currently covers Windows API modules. It does not establish Unix or kernel dispatch coverage.
