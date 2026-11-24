# mwemu

A multi-platform x86/x64/AArch64 binary emulator written in Rust. Supports PE, ELF, and Mach-O formats across Windows, Linux, and macOS.

Also implement some Windows system simulation, EPB+TEB+LDR, --syscall-mode, and also winapi implementations.

Kernel-mode too: `.ko` drivers are linked against an emulated kernel (stubs for
every import, a modelled slab allocator) so lifetime bugs — use-after-free,
double free, slab overflow, leaks — are reported instead of silently working.
See `docs/KERNEL.md`; the test target is `drivers/linux/tlm` (`make linux-driver`).

## Golden Rules

1. MWEMU should be able to emulate the maximum number of instructions possible.

2. no unsafe blocks and no unsafe dependencies or risky of supply chain attacks.

3. keep it offline except Microsoft symbols server, but also don't spam them.

4. every function 1 unique purpose

5. keep it simple, no unneeded over-complications

6. easy to use API

7. don't break any test, unless the test is bad implemented.

8. Please, use that format: make fmt  and make clippy

9. Don't do commits unless the user ask for it.

10. check TEST.md before pushing to git or when changes are ready.

11. CI cannot do tests that needs dll's that would spam the symbol server.

12. avoid unwrap, expect, panic, etc

13. Follow the [Apple Silicon test instructions](TEST.md#apple-silicon) when choosing a macOS target.


## Skills

- [api-emulation](.agents/skills/api-emulation/SKILL.md): Use when implementing, reviewing, or refactoring library API, runtime, syscall, or kernel API handlers. Read the shared checks and the applicable Windows or Linux/macOS reference.

## Testing

Run `cargo test --locked` for offline tests. Use `python scripts/sync_assets.py` before the guest fixture suite. See [TEST.md](TEST.md) for group selection.

The kernel-mode tests need the driver artefact, which is built from source
against the running kernel's headers:

```bash
make linux-driver                              # -> test/linux_uaf_driver.ko
cargo test -p libmwemu --features asset-tests tests::kernel::linux_uaf_tests -- --nocapture
```

Selected tests require their fixture. The portable suite uses the Linux module from the release bundle.

Testing in release mode could mask errors.

Check correct exe loader with IAT binding:

```bash
cargo run --release -- -f ~/soft/calc.exe -6 -v 
```

Check syscall mode:

```bash
cargo run --release -- -f ~/soft/calc.exe -6 -v --syscall-mode
```


## Verbose 

for syscall mode:

1. no verbose --> syscalls in yellow
2. -v --> also WinAPI in red and messages.
2. -vv --> also assembly
3. -vvv --> also rep iterations

for non-syscall mode:

1. no verbose --> WinAPI in red
2. -v --> also messages
3. -vv --> also assembly
4. -vvv --> also rep iterations


## Project Structure

```
crates/
  libmwemu/     Core emulation library
  mwemu/        CLI binary
  pymwemu/      Python bindings (PyO3)
  mwemu-test/   Integration tests
```

### libmwemu/src/ module layout

```
arch/           Arch enum, x86/ (regs, flags, FPU, eflags, context), aarch64/ (regs)
loaders/        Binary format parsers: elf/, macho/, pe/
threading/      Thread scheduler, thread context, crit_state, global_locks
exception/      Exception handlers (SEH/VEH/UEF), exception types
windows/        Windows internals: peb/, structures/, kuser_shared, constants
debug/          Console, script interpreter, GDB server, breakpoints, tracing, definitions
utils/          Macros, ANSI colors, helper utilities
api/            Platform API interception: windows/, linux/, macos/, banzai (unimplemented API stubs)
emu/            Core emulator state machine, execution, memory, registers, emu_context
engine/         x86 instruction dispatch (mnemonic -> handler)
maps/           Memory regions, heap allocation, TLB, permissions
syscall/        Syscall dispatch per platform (windows/, linux/, macos/)
serialization/  State serialization, minidump conversion
config.rs       Emulation configuration
err.rs          Custom error type
hooks.rs        Callback hook definitions
```
