# API emulation

mwemu can intercept library calls, execute mapped library code, or dispatch direct syscalls and kernel routines. These routes share some frame mechanics, but retain distinct calling conventions and policies.

| Surface | Current handling |
| --- | --- |
| Windows user-mode library calls | Full or partial handlers first, then the Windows unsupported-call policy below. |
| Linux/macOS library hooks | Platform handlers and their existing unimplemented-call fallback. Checked frame and memory failures stop emulation. |
| Real mapped libraries | Execute guest instructions when selected by the execution mode. A hook may be bypassed. |
| Windows syscalls and x64 driver imports | Existing handlers first, then the same Windows policy through separate entry adapters. |
| Linux/macOS syscalls and kernel routines | Their platform dispatchers and existing policies. |

A Windows import can reach a named handler even when its DLL has no mapped image. For a mapped export, mwemu resolves the address and follows PE forwarders by name or ordinal. The selected execution mode can also run the mapped guest code. See [Kernel-mode emulation](KERNEL.md) for driver imports and module loading.

## Handler ABI

Inside `libmwemu`, bind the adapter for the current entry, for example `let call = WinApi(Stdcall);`. Read arguments with methods such as `call.arg_u16(emu, 0)` and write the result with `call.set_return(emu, value)`. These adapters are internal. The former public `Emu::kernel_arg` and `Emu::set_kernel_ret` methods have been removed.

For `WinApi`, the function convention is mandatory: `Stdcall`, `Cdecl`, or non-variadic `Thiscall`. Both platforms provide signed/unsigned integer readers from 8 to 64 bits. Windows also provides `arg_f32` and `arg_f64`. Readers return `Result`. The value width does not change the ABI slot stride.

| Entry variant | Use |
| --- | --- |
| `WinApi(convention)` | Windows API hooks after the gateway consumes the return address. x86 indices count DWORD slots, x64 indices count parameter positions. |
| `WinSyscall` | The existing Windows x64 syscall interception point, with its return address still on the stack. Shared semantics receive the adapter explicitly. |
| `WinSysenter` | Existing x86 `SYSENTER` entry. EDX points to the fast-call frame, with two return addresses before the DWORD argument slots. |
| `WinInterrupt` | Existing Windows x86 `INT 0x80` compatibility route. Supports generic results, with no declared argument layout. Argument reads report `UnsupportedConvention`. |
| `UnixApi` | Linux/macOS function and kernel hooks. Fixed integer arguments use System V AMD64 register and stack slots or the supported AArch64 argument registers. |
| `LinuxSyscall` | Direct Linux syscalls. The trap selects the x86, x86-64 or AArch64 argument registers. |

Kernel dispatch consumes the x64 return slot or obtains the AArch64 link register before calling the handler. A unit-returning handler uses its platform's `api_try!` to record a fault. Internal helpers propagate errors with `?`. Read required arguments before writing outputs, allocating memory or invoking callbacks.

Result setters do not complete calls. `cleanup_args(emu, words)` discards callee-owned x86 DWORD slots without reading them. `set_return(emu, value)` only stores the result. Cleanup leaves the stack unchanged for `Cdecl`, native x64/ARM64, and syscall entries. Count stack slots rather than source parameters: an x86 64-bit integer consumes two slots.

`Thiscall` exposes the implicit object at argument index zero (ECX on x86). Explicit x86 stack slots start at index one and cleanup excludes the object. Native x64/ARM64 retain their standard layouts regardless of the source-level convention. Raw register access remains appropriate for CPU contexts and special intrinsic conventions.

## Failures and snapshots

An unreadable, overflowing, or unsupported argument layout is an emulator frame failure, distinct from a guest API error result. Windows and Unix hook failures stop `run()` with an error and prevent `step()` from advancing. They do not fabricate a return value, guest exception, `LastError`, or `errno`, or mark the process terminated. Decode arguments before side effects. The stop mechanism does not roll back arbitrary handler work already performed.

Windows retains its context in `emu.api_policy.last_outcome`. Unix stores frame diagnostics in `emu.unix_api.fault` and checked memory diagnostics in `emu.unix_api.memory_fault`. Both retain the failing library, symbol, address, entry PC/SP, and thread. Nested Unix calls retain the inner failure context. `clear_outcome()` and `clear_fault()` clear their respective diagnostics. They do not repair the frame or arrange a safe restart.

Native snapshots preserve the Windows and Unix API state represented by the current format, including timer registrations, FLS callbacks and cryptographic state. Snapshots that predate API policy restore the strict Windows default. See [native snapshot compatibility](serialization-roadmap.md#native-snapshot-compatibility).

Existing Banzai settings are preserved. Platform API state is separate from the legacy `Config` layout. Serializing `Config` alone does not retain it.

## Linux and macOS hooks

`UnixApi` models fixed integer arguments in System V AMD64 registers and stack slots. Fixed AArch64 arguments beyond its supported registers, floating-point arguments and aggregates need separate handling. For integer variadic calls, Linux AArch64 uses the remaining argument registers before the stack. Darwin AArch64 takes these arguments from the stack. Formatted output is published only after the required arguments have been read.

Linux and macOS retain their existing unknown-library and kernel fallbacks. Unsupported Linux syscalls return `-ENOSYS` after runtime hooks and handlers have had a chance to run. A raw Linux syscall leaves libc `errno` alone. Windows `ApiMode` applies only to Windows calls.

Linux and Darwin string/memory hooks share byte-oriented operations across x64 and AArch64. Bounded operations stop reading at the requested byte count or NUL where required. Checked variants validate destination object sizes. A checked hook that encounters invalid memory or exceeds an object bound records an emulator fault. This path does not deliver a Unix signal. Case-insensitive string comparisons use the existing C-locale profile.

Tests of library hooks must select the intended route. In particular, Linux real-libc execution can bypass Rust handlers.

## Windows API policy

Windows DLL dispatch uses existing full or partial handlers first. Unsupported user-mode calls, Windows syscalls, and Windows x64 driver imports use one policy. User-mode routes include unresolved imports, CRT fallback, and TLS initialization. Explicit execution of mapped DLL code retains its existing precedence.

### Modes

The default is **`Strict`**. In the CLI, `--banzai` without `--api-mode` or `--strict-api` instead selects `SimulateSuccess` and prints one warning. This does not change the library default.

| Mode | Unsupported call |
| --- | --- |
| `Strict` | Stop before publishing a fallback result or changing output buffers. |
| `SimulateFail` | Return a declared failure when its semantics are known. Otherwise use the generic zero-result fallback. |
| `SimulateSuccess` | Return modeled success when its semantics are known. Otherwise use the same generic zero-result fallback. |

The CLI accepts `--api-mode strict`, `--api-mode simulate-fail`, or `--api-mode simulate-success`. Rust exposes `ApiMode` through `libmwemu` and selects it with `emu.set_api_mode(...)`. Python uses `emu.set_api_mode("strict")`, `"simulate-fail"`, or `"simulate-success"`.

Simulation applies only after normal dispatch reports an unsupported operation. Existing handlers keep their behavior in every mode, including modeled errors and data results such as `IsDebuggerPresent`. This policy does not yet classify every unsupported branch inside partial handlers.

### Declared models

Simulation uses the declared result contract when one exists. It leaves output buffers and guest objects alone. A handler that rejects its input keeps its own error path.

| Example result contract | `SimulateFail` | `SimulateSuccess` |
| --- | --- | --- |
| Cataloged `NTSTATUS` call | `STATUS_NOT_IMPLEMENTED` | `STATUS_SUCCESS` |
| Win32 `BOOL` | `FALSE`, sets `LastError` to `ERROR_CALL_NOT_IMPLEMENTED` | `TRUE`, preserves `LastError` |
| `HRESULT` | `E_NOTIMPL` | `S_OK` |
| Direct Win32 status | Returns `ERROR_CALL_NOT_IMPLEMENTED` | Returns zero |
| Declared `void` call | Leaves the return register alone | Leaves the return register alone |
| No result model | Returns integer zero if ABI checks pass | Same behavior |

The return contract also determines the error channel. NT and `HRESULT` models preserve `LastError`. Winsock failure models return the appropriate socket sentinel and set `WSAEOPNOTSUPP`. Some calls have a fixed outcome in both modes. For example, an unsupported message read cannot supply a message.

### Syscall and kernel entry

Windows syscall handlers retain their runtime-hook precedence. Service names come from recognizable Nt export stubs in the loaded guest ntdll. The x64 dispatcher translates those numbers to its existing handler identifiers. It uses the original static profile only when no ntdll image/index or Windows build is selected. Missing or conflicting guest identities must not select an unrelated static handler. Unresolved services remain eligible for generic simulation.

The x86 `SYSENTER` path uses the loaded x86 service index and checked EDX-based argument access. The Windows `INT 0x80` path retains generic continuation without claiming a native Windows parameter layout. These adapters do not add WoW64 or `INT 0x2e` support. Syscall simulation preserves the stack and leaves the guest stub to return and clean its arguments.

Windows driver imports retain their kernel handlers and exact Nt/Zw names. Missing handlers use the shared result models or generic simulation, without routing through user-mode DLL handlers. An NT signature supplies a status-only fallback, not kernel parameter validation or a complete service implementation. Unknown calls remain listed in `KernelEnv::unimplemented`. Policy stops propagate through `kernel_call`, DriverEntry, and IRP error results. Linux/macOS kernel fallback behavior is unchanged.

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
| Windows syscall in either simulation mode | Use its known result model or generic zero, without argument cleanup | Same behavior |

If an unsupported call has only a Banzai cleanup entry, both modes behave alike. `--api-mode simulate-fail --banzai` and `--api-mode simulate-success --banzai` use that cleanup, return zero, and record `SimulatedGeneric`. The modes produce different results only when an explicit result model exists. Banzai's existing instruction-skipping and memory-diagnostic effects remain separate. Selecting an API mode does not change those flags.

No signature catalog yet replaces all cleanup entries supplied by `banzai.csv` or `banzai_add`. PE export indexes identify names and addresses, not argument layouts.

`banzai.csv` contains exact API names and x86 callee cleanup sizes in DWORD slots, excluding the return address. These are not necessarily C parameter counts: a by-value 64-bit argument uses two slots. Malformed rows are skipped and later valid duplicates replace earlier entries. Windows initialization loads the file from the configured maps folder for x86 with Banzai enabled.

Fallback also loads missing defaults on demand, so enabling Banzai after initialization works. This lazy load preserves runtime `banzai_add` overrides and restored entries. Missing or negative counts stop fallback. A known count of zero performs no argument cleanup.

### Stop and diagnostic contract

`emu.api_policy.last_outcome` records the requested name, library, route, mode and entry context. For syscalls, `call.address` holds the raw service number and `call.pc` the instruction address. For driver imports, the address identifies the import stub. Each completed simulation increments `simulation_count` and records the result and `LastError` action. `SimulatedGeneric` covers both generic zero and declared `void` returns. A `void` outcome has no result value.

`UnknownCleanup` means x86 cleanup metadata is unavailable or fails subsystem ABI checks. `BanzaiDisabled` means the optional x86 metadata fallback is disabled. Neither reason describes a missing result model.

A `LastError` action of `None` means preserve, including for stopped calls. It is not the current error value. Logs identify the selected model, including `GenericZero`. Diagnostics are bounded, not an event history.

A pending API stop prevents another run from silently continuing. `clear_outcome()` clears it explicitly but does not repair the unsupported call or arrange a safe restart. For intercepted x86/x64 calls, the retained stack is the entry frame after the guest call placed its return address and before the gateway consumed it. Direct gateway calls retain the frame supplied by their caller.

TLS callbacks use the selected policy throughout loading. A callback that stops or fails to return aborts initialization instead of silently proceeding to the entry point.

## Shared partial models

The table summarizes selected models and their limits. Guest layouts still follow the active architecture.

| Area | Behavior and limits |
| --- | --- |
| Windows files and objects | Supported Win32 and native operations share guest files, process and token handles, and waitable objects. Pipes and mailslots model local synchronous I/O. |
| Windows exceptions | x86 walks the SEH registration chain. x64 searches caller frames using version 1 unwind metadata and registered function tables. A language handler can use `RtlUnwindEx` to run termination handlers and resume at a target frame. |
| Windows timers | `SetTimer` and `KillTimer` manage thread and window timers on the guest clock. Message retrieval generates `WM_TIMER`. Dispatch checks live `TIMERPROC` registrations and invokes the guest callback. |
| Windows fiber-local storage | Each modeled Windows thread has a default fiber. FLS slots can register guest cleanup callbacks. Freeing a slot invokes its callback for nonzero values across threads. |
| Windows resources and paths | Resource and version queries use loaded images or guest files where supported. Version queries can read an en-US `.mui` sidecar. Known-folder queries use the guest profile. |
| Windows time and environment | Date and time conversions use a fixed UTC environment. Process variables and the two CRT environments have separate storage. Direct guest writes to environment tables do not update those models. |
| MSVCRT/UCRT | Startup uses guest arguments. Streams and descriptors use the configured file root. Each runtime tracks thread-local `errno` and random state. Snapshots exclude open host-backed streams. |
| Winsock | Successful startup calls increment a count. The final cleanup releases the modeled sockets. Local address and event queries work offline. The model refuses TCP connections and supplies no incoming packets. |
| Unix libraries and Linux I/O | Hooks implement byte-oriented string operations and thread-local `errno`. Linux libc and raw syscalls share pipe and event descriptors. Registering an alternate signal stack does not deliver signals. |

## Adding or moving handlers

Keep handlers in named functions and let dispatch select a handler or fallback. After a handler records a fault, return to the entry boundary so it can report the fault. Keep related A/W variants together and preserve the invoked export name in traces.

Run `make api-modules` after adding or moving sources under `api`, `syscall` or `kernel`. The check compares files on disk with compiler dependencies and separately checks Git ignore rules. Windows API modules also have declaration and re-export checks. Exercise the actual dispatch route in a test to verify reachability.
