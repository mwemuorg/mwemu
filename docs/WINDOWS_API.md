# Windows API fallback

Windows DLL dispatch uses existing full or partial handlers first. Unsupported user-mode calls use one policy on x86 and x64, including unresolved imports, CRT fallback, and TLS initialization. Explicit execution of mapped DLL code retains its existing precedence.

## Modes

The default is **`Strict`**. This changes the previous behavior of catalog-only NT calls and several library fallbacks that could continue without a handler.

| Mode | Unsupported call |
| --- | --- |
| `Strict` | Stop before publishing a fallback result or changing output buffers. |
| `SimulateFail` | Return a declared failure using a known ABI and error contract; stop when that information is missing. |
| `SimulateOptimistic` | Apply an explicit optimistic model, including its required outputs; stop when no such model exists. |

The CLI accepts `--api-mode strict`, `--api-mode simulate-fail`, or `--api-mode simulate-optimistic`. Rust exposes `ApiMode` through `libmwemu` and selects it with `emu.set_api_mode(...)`. Python uses `emu.set_api_mode("strict")`, `"simulate-fail"`, or `"simulate-optimistic"`.

Simulation applies only after normal dispatch reports an unsupported operation. Existing handlers keep their behavior in every mode, including modeled errors and data results such as `IsDebuggerPresent`. This policy does not yet classify every unsupported branch inside partial handlers. Direct syscalls and the driver kernel surface remain separate migration work.

## Declared models

| Surface | `SimulateFail` | `SimulateOptimistic` |
| --- | --- | --- |
| `NTSTATUS`-returning Nt/Zw calls in the NT signature catalog | `STATUS_NOT_IMPLEMENTED`, with unchanged output buffers, objects, and `LastError` | Only `NtYieldExecution` currently has a model: `STATUS_SUCCESS`, with no scheduler changes |
| `FileTimeToLocalFileTime` and `LocalFileTimeToFileTime` without a handler | `FALSE` and `LastError` = `ERROR_CALL_NOT_IMPLEMENTED`; no output writes | Convert in a fixed UTC environment by copying the `FILETIME` value to a checked output buffer; preserve `LastError` |
| Other unsupported calls | Stop if no declared failure model is available | Stop if no optimistic model is available |

The NT failure model uses the entire catalog, including user-mode Zw aliases, without consulting `banzai.csv`. It is an explicit unsupported-operation result before executing the API body, not a reproduction of every Windows validation error. Excluded non-`NTSTATUS` and ambiguous signatures remain excluded. Argument validation distinguishes x86 DWORD slots from native parameter positions, and x86 callee cleanup occurs exactly once. x64 retains caller-owned stack space.

Optimistic mode never silently switches to a failure model. A signature alone does not establish how to produce a valid handle, object, or output structure. The UTC conversion model rejects unreadable input, unwritable output, and invalid buffer addresses with a controlled stop before publishing success. It does not use the host time zone. Yield simulation does not promise Windows scheduler behavior.

`LastError` is part of each Win32 model, not a global consequence of success or failure. NT failures preserve it; the declared BOOL failure sets it; UTC conversion preserves it on success. Models use the existing emulator error-state helpers. Thread isolation and direct TEB synchronization of that existing state are not implemented by this policy change.

## Compatibility and metadata

Legacy `--banzai`, `enable_banzai()`, Python `enable_banzai_mode()`, and `Config::skip_unimplemented` select `SimulateFail` only when no explicit mode is set. `--strict-api` and `Config::stop_on_unimplemented` always force `Strict`. Conflicts produce a diagnostic when an unsupported call is reached. Selecting an API mode does not change instruction or memory Banzai settings.

The NT signature catalog and `banzai.csv` remain metadata. A cleanup count alone does not authorize returning zero or inventing output state. The NT catalog supplies the ABI for the declared NT failure model; non-NT simulation requires an explicit result contract.

`banzai.csv` contains exact API names and x86 callee cleanup sizes in DWORD slots, excluding the return address. These are not necessarily C parameter counts: a by-value 64-bit argument uses two slots. Malformed rows are skipped and later valid duplicates replace earlier entries. Windows initialization still loads the file from the configured maps folder for x86 with Banzai enabled; runtime `banzai_add` overrides remain available. Neither mechanism adds simulation recipes.

## Stop and diagnostic contract

Unsupported stops make `run()` return an error and `step()` stop advancing. They do not mark the guest process terminated or deliver a fabricated guest exception. The entry PC, stack pointer, thread, requested name, library, route, mode, and reason are retained in `emu.api_policy.last_outcome`. Completed simulation records its disposition, return value, and `LastError` action, and increments `simulation_count`. A `LastError` action of `None` means preserve, including for stopped calls; it is not the current error value. Diagnostics are bounded, not an event history.

A pending API stop prevents another run from silently continuing. `clear_outcome()` clears it explicitly but does not repair the unsupported call or arrange a safe restart. For intercepted x86/x64 calls, the retained stack is the entry frame after the guest call placed its return address and before the gateway consumed it. Direct gateway calls retain the frame supplied by their caller.

TLS callbacks use the selected policy throughout loading. A callback that stops or fails to return aborts initialization instead of silently proceeding to the entry point.

Native snapshots use version 4 to retain policy settings and pending outcomes. Older published native formats remain readable and derive the mode from their legacy configuration flags. The policy is stored on `Emu` separately from the legacy `Config` layout. For JSON settings, serialize `ApiPolicy` alongside `Config`; `Config` alone retains only the legacy flags.

## Adding handlers

Keep A/W variants in the same module and connect each implementation through a single module owner. Run `make api-modules` after adding or moving handlers. CI compares Windows API source files with compiler dependencies and rejects unconnected Rust files.
