---
name: windows-api-emulation
description: Implement, review, or refactor Windows API emulation handlers. Check exported names, guest calling conventions, memory and string contracts, partial implementations, dispatch reachability, and regression tests. Use for emulator WinAPI, NT API, and Windows CRT work, rather than ordinary applications that call these APIs.
---

# Windows API emulation

Use this checklist for the APIs and architectures in the requested scope. Separate confirmed defects from incomplete models and code cleanup. A working packed sample is useful regression evidence, but does not prove that an API follows its contract.

## Establish the contract and active path

- Trace resolution from the DLL or API-set import through the gateway and subsystem dispatcher to the handler. A compiled function can still be unreachable. Before removing an old implementation, compare it with the active handler and check tests, public exports, callbacks, and other callers.
- Verify the prototype, calling convention, return type, parameter directions, and size units against Microsoft Learn and SDK/WDK headers. For undocumented NT declarations, use PHNT as an additional reference; a declaration establishes an ABI, not implementation semantics. Record uncertainty when sources disagree.
- Check exact export spelling and case. SDK encoding-neutral macros are not necessarily exported symbols. Look for irregular pairs such as `getnameinfo` / `GetNameInfoW`, not just suffix substitution. An export in one DLL or Windows version does not establish that it exists in another.
- Compare supported guest architectures and A/W variants. Add a missing variant only when its export and contract are established. Local DLL exports can confirm presence, but absence on one host does not prove absence on all supported systems.
- Distinguish a real handler, a partial model, an explicit unsupported result, and a generic fallback. Preserve handler precedence. Do not claim that a fallback implements an API, or change fallback policy as an incidental cleanup.

## Adapt the guest ABI at the boundary

- Determine the guest architecture and exact interception point before calculating offsets. Check whether the engine has already removed the return address. Do not derive the guest ABI from the host architecture or a directory name.
- Prefer the existing canonical ABI interface for ordinary arguments and results. Before adding a wrapper, identify what it contributes: a distinct calling convention, a typed value, or failure handling. Remove redundant aliases only after checking all callers and preserving their types and behavior.
- Keep API-hook and syscall entry adapters distinct where their stack layout or cleanup differs. Shared semantics can accept a call-mode adapter. Do not replace that adapter with an API-only reader in shared syscall paths.
- Use raw register access when the register itself is the subject, such as context capture, unwind state, or an intrinsic convention. Ordinary parameter reads belong at the ABI boundary. Avoid mechanical replacement of every register read.

| Boundary | Checks |
| --- | --- |
| x86 stack arguments | Count DWORD slots, not source parameters. A by-value `POINT`, a 64-bit integer, or a `double` can consume two slots. Distinguish a structure passed by value from a pointer to it. |
| x86 return and cleanup | `stdcall` removes stack arguments; `cdecl` leaves them to the caller. Check register arguments for `fastcall` and `thiscall`, and special CRT/intrinsic conventions. Preserve cleanup on every exit path without removing the return address twice. |
| Windows x64 | Integer/pointer positions use `RCX`, `RDX`, `R8`, and `R9`, followed by stack arguments after shadow space. Typed floating-point positions use XMM registers. Account for the interception point and varargs rules. |
| Other supported guests | Use their own argument, floating-point, return, and stack rules. Sharing 64-bit pointer width does not make an ABI identical to x64. |

- Check result width and location: integer/pointer results, x86 `EDX:EAX` pairs, floating-point results, and hidden structure returns require different handling. Do not infer a return convention from the API name or DLL alone.
- Preserve nonvolatile registers. Read arguments before cleanup, result writes, or callbacks that could overwrite their source registers.
- Treat failed argument reads separately from arguments whose value is zero. `unwrap_or(0)` is an emulator policy that can hide a broken call frame; it does not reproduce Windows validation. Prefer fallible reads where the caller can handle failure. Do not globally replace failures with an invented error code or alter fault policy during a mechanical refactor.
- Check address and size arithmetic for overflow. An invalid guest stack or pointer must not cause a host panic.

## Respect memory, output, and error contracts

- Identify every parameter as a value, handle, input pointer, output pointer, in/out pointer, callback, or opaque context. Do not write a fabricated handle or zero through an argument merely because its position resembles an output slot in another API.
- Use guest type widths and layouts. A `DWORD` remains 32 bits on x64; pointers, `SIZE_T`, and some structures change with guest width. Check packing, alignment, unions, size fields, and reserved fields against the relevant definition.
- Check complete access ranges and arithmetic before writes. Handle optional NULL pointers separately from non-NULL invalid pointers. Preserve required access checks and memory hooks when sharing implementations.
- Verify size-query behavior, required lengths, terminators, truncation, and whether lengths include the terminator. Preserve output buffers on failure when the API requires it; do not assume every API leaves outputs untouched or always clears them.
- Match the actual success value and error channel: `BOOL`, pointer/handle sentinels, `NTSTATUS`, `HRESULT`, direct error returns, and Winsock errors are not interchangeable. A nonzero `HRESULT` is not automatically a failure.
- Update `LastError`, WSA error state, or `errno` only as the contract requires. Success does not generally imply clearing an error. Use the existing per-emulation or per-thread state mechanism.
- Keep handle creation, lookup, ownership, and release consistent. Unsupported work must not report success with unusable output objects. For a deliberately partial model, describe its supported behavior and test that behavior without claiming full Windows parity.

## Share A/W behavior without losing encoding rules

- Keep a real A/W family in one source file when that fits the module layout. Share its common behavior; use an encoding flag or existing string adapter for encoding differences. A flag is unnecessary when it changes nothing.
- Check for existing string/path helpers before adding another. Replace duplicated encoding branches with a suitable helper, but inspect its error handling, truncation, and conversion behavior first.
- Count ANSI bytes and UTF-16 code units as required by the API, not Rust characters or UTF-8 bytes. Preserve raw code units for copy operations; supplementary characters and unpaired surrogates expose accidental lossy conversions. ANSI APIs can depend on a code page and are not generally UTF-8 APIs.
- Check zero capacity, one-unit capacity, embedded NULs, bounded and unbounded forms, and signed length parameters. Do not assume that an unsuffixed export, an SDK macro, and the A variant are interchangeable.
- Preserve differences in structure layout, return counts, and validation between variants. Similar function bodies alone do not justify merging their contracts.
- Preserve the invoked export name in API tracing, including A/W and aliases. Shared logic should not make a W call appear as an A call in `log_red!` or equivalent tracing. Trace level and visibility are part of the emulator's diagnostics.

## Keep the implementation maintainable

- Reuse existing subsystem modules and semantic helpers. Split large general-purpose DLL dispatchers by subsystem; avoid creating layers for small, cohesive libraries. Do not create catchall files solely to group functions changed in the same PR.
- Use SDK/WDK names for constants and keep related definitions in coherent groups. Import constants consistently. Avoid a magic value followed by a comment that merely supplies its missing name, redundant aliases, and partial moves that scatter one constant family.
- Group explicit leaf reexports, such as `pub use family::{ApiA, ApiW};`. Wildcard reexports can be appropriate for intentionally aggregating subsystem modules; keep them visibly separate from leaf exports and check their public surface. Prefer ordinary module layout to unnecessary `#[path]` indirection.
- Keep shared implementation details private. Use context-specific names, remove redundant suffixes, and follow local formatting. Comments should explain a limitation, invariant, or surprising contract; remove stale comments and prose that only repeats the function name or code.
- Propagate meaningful read/write failures. During cleanup, check that removed writes, handle creation, or logging were actually wrong or redundant. Do not silently drop behavior to make two bodies identical.

## Validate the change

- Put regression tests in the existing subsystem test files. Use named cases and constants that explain the contract; avoid a new catchall test file named after the PR. Reuse emulator setup where isolation permits, and reset the state that each case relies on.
- Exercise the actual gateway or resolver when changing dispatch, spelling, aliases, or A/W coverage. Ensure each table-driven case invokes its own selected function, not the first export repeatedly.
- Test the relevant ABI boundaries: x86 stack delta on success and failure, unchanged nonvolatile registers, x64 arguments beyond the fourth position, pointers above 4 GiB, typed returns, and canaries around narrow output fields. Assert results and errors, not just that the handler returns without panicking.
- Cover both encodings and supported architectures for shared behavior. Include sizing queries, boundary capacities, invalid handles, and optional or invalid pointers where they affect the change. Use focused tests that demonstrate the defect rather than duplicating implementation arithmetic.
- Run the repository's prescribed formatting, lint, and test checks. Tests should not depend on host network access or download system DLLs. State skipped prerequisites and platform limits rather than treating unexecuted checks as passed.
- For unpacking regressions, compare the same fixture revision, limits, and source state. Check recovered secrets and structural output as well as termination; a successful return alone can conceal lost behavior. Keep sample execution inside the emulator.
- Record confirmed issues with the active path, expected and observed behavior, and a minimal reproduction. Keep unverified findings and model limitations distinct. Report bug fixes, refactoring, and remaining scope separately; do not describe a broad audit as a complete implementation audit without evidence.

## Contract references

Consult the per-function Microsoft Learn page and SDK/WDK declaration first. Useful ABI and naming references:

- [Argument passing and calling conventions](https://learn.microsoft.com/en-us/cpp/cpp/argument-passing-and-naming-conventions)
- [Windows x64 calling convention](https://learn.microsoft.com/en-us/cpp/build/x64-calling-convention)
- [Encoding-neutral, ANSI, and Unicode prototypes](https://learn.microsoft.com/en-us/windows/win32/intl/conventions-for-function-prototypes)
