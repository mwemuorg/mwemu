# Linux and macOS API checks

Apply these checks alongside the shared [API emulation guidance](../SKILL.md).

## Library hooks and native execution

- Identify whether the call reaches a libc/libSystem hook, executes real mapped library code, or enters a syscall handler. Confirm the configured route in tests. A real-libc mode can bypass the hook being tested.
- Verify the exported symbol and any platform naming or versioning rules before normalizing it. Keep requested and resolved names available for diagnostics, especially when a gateway dispatches several libraries through one implementation.
- Read the applicable libc, OS, or kernel declaration. Similar names across Linux, Darwin, and Windows CRT do not establish matching layouts, error behavior, or ownership.

## Function and variadic ABI

- Check System V AMD64 and the target ARM64 variant separately. Establish whether a hook sees the return address on the stack or already consumed. Use the function adapter for library calls, not a syscall register layout.
- Keep fixed-argument and variadic placement distinct. For the integer varargs modeled here, Linux ARM64 uses remaining argument registers before stack slots. Darwin ARM64 variadic arguments use the stack. Do not infer arbitrary fixed-argument stack packing, floating-point placement, or aggregate classification from this limited model.
- Verify C default promotions, signedness, alignment, and register exhaustion. A formatter's shared parsing logic can accept a fallible argument reader. It should propagate a frame error before publishing output. A missing argument is not a formatted zero, and a format with no argument conversions should not read the stack unnecessarily.
- Preserve calling context through guest callbacks, libc startup, and thread routines. A failed nested guest call must not become a successful outer return or process exit merely because an internal helper returned early.

## Errors and emulated resources

- Distinguish the public library result, its `errno` behavior, and the underlying syscall result. A broken emulator call frame is not a reason to invent `errno` or borrow Windows `LastError` policy.
- Check how `errno` is stored and retrieved for the current emulated thread. Preserve it when the contract requires. Do not assume every successful libc call clears it.
- Keep descriptor, stream, mapping, allocation, process, and thread ownership consistent across related calls. Preserve supported short-I/O, EOF, termination, wait, and synchronization behavior. Do not replace them with universal zero-return stubs during ABI cleanup.
- Follow the subsystem's host-isolation rules. Review existing host-backed reads separately from simulated guest writes. Sharing host APIs or structs can change both side effects and guest layout.

## Syscalls and kernel API calls

- Treat a syscall instruction boundary, a libc syscall wrapper, and a direct call to a kernel routine as separate contracts. Check number resolution, register clobbers, stack layout, result width, error flags or status conventions, and supported guest architecture at the actual interception point.
- Do not copy a user-mode generic fallback or stop policy into kernel handlers without checking their existing dispatcher and resource model. Shared frame mechanics can be reused while fault policy stays at the appropriate boundary.
- Driver callbacks and kernel routines may have context, ownership, blocking, or scheduling constraints beyond ordinary library calls. Verify the relevant Linux kernel or Darwin contract and the emulator's supported model instead of claiming coverage through an adapter alone.

These checks do not imply that the emulator already implements complete Unix ABI classification, every syscall convention, thread-local error state, or a unified kernel policy. Record remaining limits explicitly.
