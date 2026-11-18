---
name: api-emulation
description: Implement, review, or refactor emulated library APIs, runtime calls, syscalls, and kernel API handlers. Check active dispatch, guest ABI, memory and error contracts, partial models, and regression coverage across Windows, Linux, and macOS.
---

# API emulation

Use this guidance for the API surface and architectures in the requested scope. Separate a confirmed defect, an incomplete model, a fallback policy, and code cleanup. A sample running farther is useful evidence, but does not establish API correctness.

Read the applicable reference alongside the shared checks:

- [Windows](references/windows.md): Win32, NT, CRT, A/W families, Windows error channels, and driver distinctions.
- [Linux and macOS](references/unix.md): libc/libSystem hooks, Unix function and variadic conventions, `errno`, direct syscalls, and kernel boundaries.

For instruction, loader, memory, or exception defects beneath the handler, use the repository's hardening guidance when available. Avoid expanding an API fix into an unrelated subsystem audit.

## Establish the contract and active path

- Trace name resolution, imports or syscall numbers, gateway selection, dispatch, and the actual handler. Verify that a compiled implementation is reachable. A source file or a matching name alone proves neither coverage nor precedence. Before deleting disconnected code, check whether it contains valid behavior missing from the active implementation.
- Identify the guest OS, architecture, privilege level, calling convention, and interception point. Distinguish a library function hook, real library execution, a syscall boundary, and a call to a kernel routine. Shared semantics may serve several entry adapters, but their frames and failure rules can differ.
- Verify names, prototypes, return types, parameter directions, widths, and version differences against platform documentation and headers. A declaration establishes layout and ABI, not full runtime behavior. Keep uncertain or implementation-defined behavior explicit.
- Distinguish a full handler, a partial model, an explicit unsupported result, and generic simulation. Preserve handler precedence and existing fallback policy unless changing it is the task. Do not present a generic return as an implementation or let a failed explicit model silently retry as success.

## Adapt the ABI at the boundary

- Use the canonical platform adapter for ordinary arguments and results. Bind it once at handler entry and select the calling convention explicitly where required. Before adding a wrapper, identify its distinct responsibility. Remove redundant aliases only after checking all callers.
- Keep reusable frame reads and scalar storage separate from platform placement and calling conventions. Shared semantics can accept an entry adapter. Do not force function hooks, syscalls, drivers, and runtimes through one layout merely because their pointers have the same width.
- Check whether interception already consumed the return address. Count ABI slots rather than source parameters. By-value aggregates, wide integers, floating-point values, varargs, and hidden parameters may need distinct placement. Preserve slot stride when reading a narrower value.
- Use typed readers when widths or signedness matter. `From` expresses lossless conversion. Narrowing and signed reinterpretation can deliberately require `as`. `TryFrom` adds a rejection path and is not a cosmetic replacement. Preserve required sign extension and partial-register behavior.
- Determine who owns argument cleanup and perform it exactly once. Discarding callee-owned slots advances the stack pointer without reading their contents. Read required arguments before cleanup, output writes, allocation, result writes, or callbacks that could overwrite the frame.
- Preserve nonvolatile registers and the correct result location: scalar registers, register pairs, floating-point/vector state, or a hidden output buffer. Raw register access is appropriate for context capture, unwind state, and intrinsic conventions whose registers are themselves the subject.
- Treat a failed frame read separately from a readable zero or NULL. Check address arithmetic and complete read ranges, including adjacent mappings and holes. Do not hide errors with `unwrap_or(0)`, an empty value, or invented platform status during a refactor.
- At a handler boundary, an error macro may record a fault and return. Internal fallible helpers should propagate it with `?`. Verify that the failure reaches `run` and `step`, survives nested calls, and retains the failing entry context. An early handler return or cleared run flag alone can look like normal termination. Keep emulator frame failures separate from guest API errors and exceptions.

## Preserve memory and environment contracts

- Classify each parameter as a value, input pointer, output pointer, in/out pointer, handle, callback, or opaque context. Do not write through an argument or create a handle because its position resembles another API's signature.
- Follow guest structure layout, packing, alignment, field widths, and size units. Native host types and layouts are not automatically guest types. Keep API-specific fixed-width integers distinct from pointers and native-sized values.
- Check arithmetic and the entire relevant memory range before publishing outputs. Distinguish optional NULL pointers from invalid non-NULL pointers. Test size queries, capacities, terminators, truncation, and canaries. Preserve outputs on failure when the contract requires it. Keep access protections and memory hooks when sharing implementations.
- Respect the actual success sentinel and error channel. Failure may use a status, a negative result, a null or invalid handle, flags, or thread-local state. Success does not universally clear an error. Verify the platform contract before changing either the result or error state.
- Keep handles, descriptors, allocations, callbacks, and release operations consistent with their modeled ownership. A partial success must produce the state needed by its supported follow-up calls. Preserve emulated side effects and host-isolation boundaries. Do not turn a guest operation into a host operation as an incidental refactor.
- Share closely related variants when their contracts agree, while preserving encoding, layout, and optional-parameter differences. Keep the invoked export or symbol name in logging. Common implementation code must not conceal the selected variant.

## Keep the change maintainable

- Reuse subsystem modules and semantic helpers. Split large general-purpose dispatchers by responsibility. Keep small cohesive libraries together. Avoid catchall helper or test files grouped only by the current PR, and keep shared implementation details private.
- Use coherent platform constant groups, meaningful type names, and consistent imports. Avoid a magic value followed by a comment that merely names it, partial constant moves, redundant aliases, or comments that restate the code.
- Group explicit leaf reexports. Wildcard reexports can suit intentional aggregation modules. Check module ownership, reachable dispatch, case-sensitive paths, ignored sources, and the publishable file set after moves.
- Keep tracing and error diagnostics at the appropriate boundary. Before removing output writes, cleanup, handle creation, or logging, establish whether the behavior is wrong or redundant. Similar-looking implementations alone do not justify removal.

## Validate the route and behavior

- Add regressions to existing subsystem test groups where possible. Use named cases and reusable setup without leaking state between cases. Test the contract rather than reproducing implementation arithmetic.
- Exercise the actual gateway or syscall path for dispatch changes. Check that each table case selects its intended handler. Confirm that the execution mode reaches that handler.
- Cover meaningful boundaries: stack/register arguments, slot counts, wide pointers, promoted or signed values, result widths, nonvolatile state, zero versus unreadable values, optional pointers, exact capacities, and output canaries. When variants share code, test their encodings and supported architectures.
- For controlled stops, check retained PC/SP and call context. Verify required output and error-state preservation in normal, cached, and threaded execution. If snapshots store pending faults, test restoration. Distinguish a handled guest exception from an emulator failure and a normal guest return.
- Run the repository's focused debug tests and required formatting, lint, and module checks without fetching missing runtime prerequisites implicitly. Report prerequisite skips and unsupported layouts separately from successful checks.
- Keep confirmed findings, unverified candidates, and model limits distinct. Record the active path, expected versus observed behavior, reproduction, fix, and remaining scope. End-to-end fixture recovery supplements handler tests. It does not replace them.
