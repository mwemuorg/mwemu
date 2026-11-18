# Windows API fallback

Windows DLL dispatch uses existing full or partial handlers first. Unsupported user-mode calls use one policy on x86 and x64, including unresolved imports, CRT fallback, and TLS initialization. Explicit execution of mapped DLL code retains its existing precedence.

## Modes

The default is **`Strict`**. In the CLI, `--banzai` without `--api-mode` or `--strict-api` instead selects `SimulateSuccess` and prints one warning. This does not change the library default.

| Mode | Unsupported call |
| --- | --- |
| `Strict` | Stop before publishing a fallback result or changing output buffers. |
| `SimulateFail` | Return a declared failure when its semantics are known; otherwise use the generic zero-result fallback. |
| `SimulateSuccess` | Return modeled success when its semantics are known; otherwise use the same generic zero-result fallback. |

The CLI accepts `--api-mode strict`, `--api-mode simulate-fail`, or `--api-mode simulate-success`. Rust exposes `ApiMode` through `libmwemu` and selects it with `emu.set_api_mode(...)`. Python uses `emu.set_api_mode("strict")`, `"simulate-fail"`, or `"simulate-success"`.

Simulation applies only after normal dispatch reports an unsupported operation. Existing handlers keep their behavior in every mode, including modeled errors and data results such as `IsDebuggerPresent`. This policy does not yet classify every unsupported branch inside partial handlers. Direct syscalls and the driver kernel surface remain separate migration work.

## Declared models

| Surface | `SimulateFail` | `SimulateSuccess` |
| --- | --- | --- |
| `NTSTATUS`-returning Nt/Zw calls in the NT signature catalog | `STATUS_NOT_IMPLEMENTED`, with unchanged output buffers, objects, and `LastError` | `STATUS_SUCCESS`, with unchanged output buffers, objects, scheduler state, and `LastError` |
| `FileTimeToLocalFileTime` and `LocalFileTimeToFileTime` without a handler | `FALSE` and `LastError` = `ERROR_CALL_NOT_IMPLEMENTED`; no output writes | Convert in a fixed UTC environment by copying the `FILETIME` value to a checked output buffer; preserve `LastError` |
| Other unsupported calls | Generic zero result, subject to ABI checks | The same generic zero result, subject to ABI checks |

Both simulation modes use the entire NT catalog, including user-mode Zw aliases. Failure simulation reports an unsupported operation before executing the API body; success simulation returns success without implementing that body. Argument validation distinguishes x86 DWORD slots from native parameter positions, and x86 callee cleanup occurs exactly once. x64 retains caller-owned stack space.

Both simulation modes prefer their declared result model and use the same generic fallback when it is absent. Generic fallback writes zero to the integer return register and preserves output buffers, objects, and `LastError`; zero may mean success, failure, or a null value depending on the API. This best-effort result does not claim an API implementation. The UTC conversion model rejects unreadable input, unwritable output, and invalid buffer addresses with a controlled stop before publishing success. An explicit model that fails validation does not retry through the generic fallback. UTC conversion does not use the host time zone.

`LastError` is part of each Win32 model, not a global consequence of success or failure. NT result models preserve it; the declared BOOL failure sets it; UTC conversion preserves it on success. Models use the existing emulator error-state helpers. Thread isolation and direct TEB synchronization of that existing state are not implemented by this policy change.

## API policy and Banzai

API mode and Banzai control different mechanisms. The CLI uses `SimulateSuccess` when `--banzai` is present without an explicit policy, and warns once on stderr that modeled success may be returned without implementing the operation. An explicit `--api-mode` takes precedence and suppresses the warning; `--strict-api` always forces strict behavior and suppresses it as well, regardless of argument order. The selected CLI policy also overrides a restored snapshot's policy; without a policy flag or `--banzai`, restoration keeps the saved policy. Rust `enable_banzai()`, Python `enable_banzai_mode()`, and `Config::skip_unimplemented` do not choose an API mode. `Config::stop_on_unimplemented` always forces strict behavior.

A subsystem's signature catalog or explicit model supplies ABI information first. Where the subsystem delegates x86 cleanup lookup to generic fallback, Banzai enables CSV or runtime metadata as the last source. That metadata supplies only a cleanup size; it cannot override subsystem ABI checks or define success/failure semantics. On x64 and AArch64 the caller owns the argument area, so generic simulation needs no Banzai cleanup.

| Unsupported call | Banzai disabled | Banzai enabled |
| --- | --- | --- |
| Any API in `Strict` | Stop | Stop |
| Known model in either simulation mode | Apply its failure/success semantics and ABI | Same behavior |
| x86 call delegated to generic cleanup in either simulation mode | Stop with `BanzaiDisabled` | Use a known cleanup size, otherwise stop with `UnknownCleanup` |
| Generic x64/AArch64 call in either simulation mode | Return zero without argument cleanup | Same behavior |

For an unsupported call with only a Banzai cleanup entry, both `--api-mode simulate-fail --banzai` and `--api-mode simulate-success --banzai` perform the same cleanup, return zero, and record `SimulatedGeneric`. The modes produce different results only when an explicit result model exists. Banzai's existing instruction-skipping and memory-diagnostic effects remain separate; selecting an API mode does not change those flags.

No signature catalog yet replaces all cleanup entries supplied by `banzai.csv` or `banzai_add`. PE export indexes identify names and addresses, not argument layouts.

`banzai.csv` contains exact API names and x86 callee cleanup sizes in DWORD slots, excluding the return address. These are not necessarily C parameter counts: a by-value 64-bit argument uses two slots. Malformed rows are skipped and later valid duplicates replace earlier entries. Windows initialization loads the file from the configured maps folder for x86 with Banzai enabled. Fallback also loads missing defaults on demand, so enabling Banzai after initialization works; this lazy load preserves runtime `banzai_add` overrides and restored entries. Missing or negative counts stop fallback; a known count of zero performs no argument cleanup.

## Stop and diagnostic contract

Unsupported stops make `run()` return an error and `step()` stop advancing. They do not mark the guest process terminated or deliver a fabricated guest exception. The entry PC, stack pointer, thread, requested name, library, route, mode, and reason are retained in `emu.api_policy.last_outcome`. Completed simulation records its disposition, return value, and `LastError` action, and increments `simulation_count`. `SimulatedGeneric` identifies a universal stub separately from modeled failure or success. `UnknownCleanup` means x86 cleanup metadata is unavailable or rejected by subsystem ABI checks, and `BanzaiDisabled` means the optional x86 metadata fallback is not enabled; neither means a missing result model. A `LastError` action of `None` means preserve, including for stopped calls; it is not the current error value. Logs identify the selected model, including `GenericZero`. Diagnostics are bounded, not an event history.

A pending API stop prevents another run from silently continuing. `clear_outcome()` clears it explicitly but does not repair the unsupported call or arrange a safe restart. For intercepted x86/x64 calls, the retained stack is the entry frame after the guest call placed its return address and before the gateway consumed it. Direct gateway calls retain the frame supplied by their caller.

TLS callbacks use the selected policy throughout loading. A callback that stops or fails to return aborts initialization instead of silently proceeding to the entry point.

Native snapshots use version 4 to retain policy settings and pending outcomes. Older published native formats remain readable and preserve their Banzai configuration; without a stored API policy, the mode defaults to strict. The policy is stored on `Emu` separately from the legacy `Config` layout. For JSON settings, serialize `ApiPolicy` alongside `Config`; `Config` alone retains only the legacy flags.

## Adding handlers

Keep A/W variants in the same module and connect each implementation through a single module owner. Run `make api-modules` after adding or moving handlers. CI compares Windows API source files with compiler dependencies and rejects unconnected Rust files.
