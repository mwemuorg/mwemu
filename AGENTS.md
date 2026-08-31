# mwemu

A multi-platform x86/x64/AArch64 binary emulator written in Rust. Supports PE, ELF, and Mach-O formats across Windows, Linux, and macOS.

Also implement some Windows system simulation, EPB+TEB+LDR, --syscall-mode, and also winapi implementations.

Kernel-mode too: `.ko` drivers are linked against an emulated kernel (stubs for
every import, a modelled slab allocator) so lifetime bugs — use-after-free,
double free, slab overflow, leaks — are reported instead of silently working.
See `docs/KERNEL.md`; the test target is `drivers/linux/tlm` (`make driver`).

## Golden Rules

1. MWEMU should be able to emulate the maximum number of instructions possible.

2. no usafe blocks and no unsafe dependencies or risky of supply chain attacks.

3. keep it offline except microsoft symbols server

4. every function 1 unique purpose

5. keep it simple, no unneeded overcomplications

6. easy to use API

7. don't break any test, unless the test is bad implemented.


## Testing

`cargo test` (if there is no test folder `make tests` will download it)

The kernel-mode tests need the driver artefact, which is built from source
against the running kernel's headers:

```bash
make driver                                    # -> test/linux_uaf_driver.ko
cargo test -p libmwemu tests::kernel -- --nocapture
```

They skip themselves when it is absent, so CI stays green without kernel headers.

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


## Building & Testing on Apple Silicon

**This is critical.** On Apple Silicon (arm64) hosts, always build and test targeting `x86_64-apple-darwin`:

```bash
cargo check --target x86_64-apple-darwin
cargo test --target x86_64-apple-darwin
cargo build --target x86_64-apple-darwin
```

The test suite produces worse results when the aarch64 host natively compiles and runs the x86_64 emulation code. When we instead tell the OS/CPU to run in x86_64 mode (via Rosetta), tests behave correctly. The root cause is not fully understood yet — it may be related to how the Rust compiler generates code differently on aarch64 vs x86_64 for our emulation logic.

**Never run bare `cargo test` or `cargo check` on an Apple Silicon host** without the `--target x86_64-apple-darwin` flag.

Ensure the target is installed:
```bash
rustup target add x86_64-apple-darwin
```

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
