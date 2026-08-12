# libmwemu examples

Runnable Rust programs showing how to drive the emulator as a library.

> Not to be confused with the repo-root `examples/` directory, which holds
> `.mwemu` scripts for the CLI's `-x` flag (deprecated in favour of pymwemu).

They sit behind the `examples` feature so a plain `cargo build` or `cargo test`
doesn't pay for compiling them. Run any of them with:

```sh
cargo run -p libmwemu --features examples --example 01_shellcode
```

Without the feature Cargo tells you what's missing rather than failing oddly:

```
error: target `01_shellcode` in package `libmwemu` requires the features: `examples`
```

| example | what it shows |
|---|---|
| `01_shellcode` | build an emulator, run raw bytes, read registers back |
| `02_memory` | map memory, read/write scalars and strings, search for patterns |
| `03_hooks` | instrument execution: count instructions, trace memory access |
| `04_load_binary` | load a real ELF/Mach-O/PE from disk and run it under a step budget |

`04_load_binary` takes arguments:

```sh
cargo run -p libmwemu --features examples --example 04_load_binary -- /bin/ls
cargo run -p libmwemu --features examples --example 04_load_binary -- test/exe64win_msgbox.bin 300000
```

## Things worth knowing

- `emu64()`, `emu32()` and `emu_aarch64()` are the three constructors.
- `load_code()` sniffs the format and sets up the address space, entry point and
  stack; `load_code_bytes()` treats the input as raw shellcode.
- `step()` runs one instruction. `run(None)` runs until the program stops,
  `run(Some(addr))` until an address is reached, and `run_to(n)` until `n`
  instructions have been emulated.
- `run()` and `run_to()` return the **program counter** where emulation stopped.
  The instruction count is `emu.pos`.
- Windows samples need `emu.cfg.maps_folder` pointing at a folder of system
  DLLs, or `emu.set_maps_from_winver("win11")` to fetch them on demand.
