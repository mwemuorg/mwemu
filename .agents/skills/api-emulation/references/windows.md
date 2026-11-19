# Windows API checks

Apply these checks alongside the shared [API emulation guidance](../SKILL.md).

## Names, declarations, and dispatch

- Verify prototypes against Microsoft Learn and SDK/WDK declarations. PHNT is an additional reference for undocumented NT declarations. Check exact export spelling and case. Encoding-neutral SDK macros are not necessarily exported symbols, and one DLL or Windows version does not establish another's exports.
- Check ordinary and irregular encoded pairs, such as `getnameinfo` / `GetNameInfoW`, and applicable Nt/Zw aliases. A user-mode Nt/Zw mapping does not establish equivalent kernel-mode behavior. Preserve handler precedence and distinguish mapped DLL execution from Rust interception.
- Keep import diagnostics and dynamic symbol lookup consistent with active dispatch. Prefer deriving discovery metadata from the same declarations, and test omitted aliases through the real resolver. A discovery list does not establish full handler semantics.
- When checking local DLL exports, presence can establish a name. Absence on one host does not prove absence on every supported target. Confirm aliases and signatures before adding a variant.

## Windows calling conventions

| Boundary | Check |
| --- | --- |
| x86 arguments | Count DWORD slots. A by-value `POINT`, wide integer, or `double` can consume two. A pointer to that value consumes one. Account for implicit objects and register arguments in `thiscall` or `fastcall`. |
| x86 return and cleanup | `stdcall` callee cleanup and `cdecl` caller cleanup differ. Check each exit path and avoid consuming the return address twice. Validate integer pairs, x87 results, and special CRT/intrinsic conventions separately. |
| Windows x64 hooks | Integer/pointer positions use `RCX`, `RDX`, `R8`, `R9`, then stack slots after shadow space. Floating-point positions and varargs need their own checks. Account for the engine's interception point. |
| Windows syscall entry | Do not copy function-hook offsets or assume the engine intercepted after every architectural register change. Verify the live syscall entry path and adapter. |
| Other guests and drivers | Use the supported guest's own ABI and privilege boundary. Sharing pointer width does not make Windows ARM64, x64, and driver calls interchangeable. |

Check public bindings and callbacks when changing an ABI helper. Do not migrate a CPU-context operation as if it were ordinary argument decoding.

Syscall numbers depend on the guest build. Resolve them from the loaded guest before using a static profile. An unresolved service number must not select an unrelated handler. Keep its raw number and entry PC in diagnostics.

A syscall instruction retains the guest stub frame. Leave function return and callee cleanup to that stub. Check architecture predicates before selecting a layout, since a family predicate can include both x86 and x64.

## Return values, error state, and ownership

- `BOOL`, pointers, handle sentinels, `NTSTATUS`, `HRESULT`, direct error returns, and Winsock errors have different success tests. A nonzero `HRESULT` is not automatically failure.
- Update `LastError`, Winsock error state, or CRT `errno` only as the API requires. NT results do not automatically set Win32 `LastError`, and success does not generally clear it. Check per-thread storage and any TEB synchronization rather than assuming a global helper models them.
- Use guest field widths: a `DWORD` remains 32 bits on x64. Pointers, `SIZE_T`, and structures may differ. Verify output widths, buffer ownership, query-size behavior, and handle lifetime through related APIs.
- Unsupported partial work must not claim success with unusable outputs. Describe and test the behavior the model supports without claiming full Windows parity or silently changing the configured unsupported-call policy.

## A/W and string families

- Keep a related A/W family in one module when appropriate. Share common behavior through an encoding flag or existing adapter, while retaining differences in signatures, structure layouts, return counts, and validation.
- Count bytes or UTF-16 code units as required. Preserve raw units in copy operations: supplementary characters and unpaired surrogates expose accidental lossy conversion. ANSI APIs can depend on a code page and are not universally UTF-8.
- Check size-query results, signed lengths, zero/one capacity, terminators, embedded NULs, and truncation. An unsuffixed export, an SDK macro, and an A variant are not interchangeable without evidence.
- Preserve the invoked A/W name in tracing, including aliases. Do not make a W call appear as A merely because both use a common implementation.

## References

- [Argument passing and naming conventions](https://learn.microsoft.com/en-us/cpp/cpp/argument-passing-and-naming-conventions)
- [Windows x64 calling convention](https://learn.microsoft.com/en-us/cpp/build/x64-calling-convention)
- [Encoding-neutral prototypes](https://learn.microsoft.com/en-us/windows/win32/intl/conventions-for-function-prototypes)

Use the per-function documentation and applicable SDK/WDK declaration for the API under review. These overview pages do not define every contract.
