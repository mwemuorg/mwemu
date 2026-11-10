# mwemu

A multi-platform x86/x64/AArch64 binary emulator written in Rust. Supports PE, ELF, and Mach-O formats across Windows, Linux, and macOS.

Also implement some Windows system simulation, EPB+TEB+LDR, --syscall-mode, and also winapi implementations.

Kernel-mode too: `.ko` drivers are linked against an emulated kernel (stubs for
every import, a modelled slab allocator) so lifetime bugs — use-after-free,
double free, slab overflow, leaks — are reported instead of silently working.
See `docs/KERNEL.md`; the test target is `drivers/linux/tlm` (`make driver`).

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

The test suite used to produce worse results when the aarch64 host natively
compiled and ran the x86_64 emulation code. Root cause (found 2026-10-28): the
SSE/AVX float handlers fed the raw operands to the *host* FPU, and hosts
disagree on the NaN they generate for an invalid operation (x86 returns the
negative "real indefinite" `0xffc00000`, AArch64 the positive default NaN
`0x7fc00000`). Those handlers now implement the x86 NaN rules themselves
(`engine/instructions/fp.rs`) and the full suite passes natively on Apple
Silicon. Keep the `--target x86_64-apple-darwin` habit anyway: it is what the
Makefile and CI assume, and any float path that still goes through the host
FPU (FMA, `cvt*`, `round*`, x87) is only guaranteed to match the silicon
under Rosetta. If a test passes under Rosetta and fails natively, that is the
first place to look.

**Never run bare `cargo test` or `cargo check` on an Apple Silicon host** without the `--target x86_64-apple-darwin` flag.

`pymwemu` additionally needs Python >= 3.10 at build time (`pyo3-stub-gen`),
and macOS ships 3.9 as `python3`. Its build script stops with a message naming
the interpreter pyo3 found; fix it with `PYO3_PYTHON=$(brew --prefix)/bin/python3.14`
or `make -C crates/pymwemu setup-env`, which picks the newest `python3.x` on
PATH for the venv.

Ensure the target is installed:
```bash
rustup target add x86_64-apple-darwin
```

## Project Structure

```
crates/
  libmwemu/         The library people depend on: re-exports the crates below, registers the layers and the WinAPI handlers, holds the test suite
  mwemu-core/       Emulator core (everything in the module layout below)
  mwemu-linux/      Linux layer: syscall/linux, api/linux (libc hooks)
  mwemu-macos/      macOS layer: syscall/macos, api/macos (libSystem hooks)
  mwemu-windows/    Windows layer: syscall/windows (NT syscalls), api/windows/common (helpers shared by the handlers), emulated registry/environment/profile/window state
  mwemu-debug/      Debugger layer: console commands, script interpreter, GDB server, tracing, serialization/ (snapshots, minidump)
  mwemu-winapi32/   32-bit Windows API handlers, one module per DLL, one file per API
  mwemu-winapi64/   64-bit Windows API handlers, same layout
  mwemu/            CLI binary
  pymwemu/      Python bindings (PyO3)
  mwemu-test/   Integration tests
```

The layer and handler crates depend on mwemu-core, never the other way round.
The core reaches the layers through the traits in `layers` (`LinuxLayer`,
`MacosLayer`, `WindowsLayer`, `DebugLayer`) and the handlers through
`api::dispatch`. `libmwemu::register_api_handlers()` fills both (`emu32()`,
`emu64()` and `emu_aarch64()` call it). Keep it that way — an edit to a
syscall, an API hook or the debugger then recompiles one small crate instead
of the core and everything built on it. When the core needs something new from
a layer, add a method to that layer's trait.

The layer crates keep the module paths their code had in mwemu-core
(`mwemu_linux::syscall::linux`, `mwemu_windows::api::windows::common`, ...).

### mwemu-core/src/ module layout

```
arch/           Arch enum, x86/ (regs, flags, FPU, eflags, context), aarch64/ (regs)
threading/      Thread scheduler, thread context, crit_state, global_locks
exception/      Exception handlers (SEH/VEH/UEF), exception types
windows/        Windows internals: peb/, structures/, kuser_shared, constants, loader32/ + loader64/ (LoadLibrary, export resolution), module_names, export index
debug/          Breakpoints, definitions, console prompts (`Console::spawn_console` opens the console in mwemu-debug)
utils/          Macros, ANSI colors, helper utilities
api/            Platform API interception: dispatch (entry points into the WinAPI handler crates), windows/helper (handle table shared by all layers), banzai (unimplemented API stubs), abi, printf, strconv
layers.rs       Traits the layer crates implement, and their registration
emu/            Core emulator state machine, execution, memory, registers, emu_context, loaders/ (PE, ELF, Mach-O, .ko/.kext)
engine/         x86 instruction dispatch (mnemonic -> handler)
maps/           Memory regions, heap allocation, TLB, permissions
config.rs       Emulation configuration
err.rs          Custom error type
hooks.rs        Callback hook definitions
```
