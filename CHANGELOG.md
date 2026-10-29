# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **Self-modifying code**: the decoded-instruction cache is now kept coherent
  with every store into executable memory. Only scalar GPR stores invalidated
  it before; vector stores (`movdqu`/`movaps`…), the `rep movs`/`rep stos`
  bulk fast path and host-side writes from API stubs (ReadFile,
  WriteProcessMemory, memcpy…) executed stale instructions when a packer
  decrypted over already-executed code. `Maps` now logs writes that hit an
  executable map and the run loop applies them (page-granular on x86, with a
  refetch when the block being executed was modified).
- **Instruction cache index out of bounds**: two blocks in a page whose index
  ended in `0xffe1..0xffff` probed past the end of the cache-entry table and
  panicked. Lines are now disjoint and bounded.
- **Packed SSE float arithmetic**: `addps/subps/mulps/addpd/subpd/mulpd`
  added, subtracted and multiplied the raw bit patterns as integers.
- **Host-independent NaN semantics**: SSE/AVX `add/sub/mul/div/sqrt`
  (`ps/pd/ss/sd` and the VEX forms) now implement the x86 NaN rules (SDM
  Vol. 1 Table 4-7) instead of relying on the host FPU. On Apple Silicon the
  host generated a positive default NaN (`0x7fc00000`) where x86 produces the
  negative real indefinite (`0xffc00000`); this was the cause of the "tests
  behave differently on aarch64" note in AGENTS.md.
- **`pcmpgtw`** compared words unsigned; **`pcmpgtb`/`pcmpgtw`** aborted the
  process (release `assert!`) on the MMX `mm` register forms. Both are signed
  now and handle 64-bit and 128-bit operands.
- **`ntdll!RtlRaiseStatus` detection** compared against a hardcoded RVA
  (`0x106fd0`) that matches none of the shipped ntdll builds (`0xdcf0`);
  the address is now resolved from the mapped ntdll's export table.
- **Loop iteration counter** (`--loops`) was O(n²): it kept every executed
  address in a `Vec` and rescanned it per instruction.
- Release-mode `assert!` in instruction handlers converted to
  `debug_assert!`; `scripts/panic-gate.sh` now counts `assert!` too.
- **`imul` two/three-operand CF/OF**: were derived from the unsigned
  magnitude, so every negative product that fits raised CF=OF and positive
  products in [2^(n-1), 2^n) did not. Now set exactly when the signed product
  does not fit the destination.
- **`imul r/m16` one-operand** zero-extended its operands (`as i32` on a u64):
  `-1 * n` produced a wrong DX:AX.
- **`div` quotient overflow** now raises #DE like the CPU instead of writing
  a truncated quotient and setting OF.
- **`loop`/`loope`/`loopne`** chose CX/ECX/RCX from the magnitude of the
  target address; in 64-bit code below 4 GB they counted ECX and cleared the
  upper half of RCX. The width now follows the address-size prefix.
- **`jecxz`** tested CX instead of ECX.
- **`scasw`/`scasd`/`scasq`** compared the accumulator with itself (iced
  places AX/EAX/RAX in operand 0), so `repne scasd` stopped on the first
  element.
- **`bt`/`bts`/`btr`/`btc` with a memory base and a register offset** reduced
  the offset modulo the operand size; the CPU addresses `base + offset/8`
  (signed), reaching the neighbouring elements of a bit array.
- **REP lost mid-instruction**: when the decoded block ended on a REP string
  instruction (always the case for shellcode loaded with `load_code_bytes`),
  the run loop resumed it with a zero instruction size and re-ran it until
  RCX reached zero; `repe/repne cmps/scas` only stopped after two consecutive
  terminating elements.
- **`step()` and REP**: a single step executed one element and moved past the
  instruction without touching RCX; it now runs the whole REP like the CPU.
- 16-bit `shrd`/`shld` with a count above 16 underflowed the shift amount
  (debug-build panic); the undefined count is now clamped.
- `Flags::shl1p8/16/32` read CF/OF from the wrong bit and `sar1p8` zero-extended;
  the one-count helpers now delegate to the two-operand forms.
- **`enter size, level`** read the level from the low byte of the size:
  `get_operand_value` returned the first immediate for `Immediate8_2nd`
  operands (also wrong for `extrq`/`insertq`). The handler now follows the
  SDM (no extra push at level 0, outer frame pointers copied from memory).
- **REP fast path with huge counts** (`rcx = -1`, the strlen idiom)
  overflowed the byte count and end address: debug-build panic, release
  could index past the map. Overflowing counts fall back to the per-element
  path.
- `daa` masks the adjusted AL and, like the new `das`, sets SF/ZF/PF from it.

### Added

- **pymwemu builds on macOS**: `python3` there is 3.9 and `pyo3-stub-gen`
  needs 3.10+, which failed with a cryptic `PyEncodingWarning` error inside
  the dependency. `build.rs` now checks the interpreter pyo3 picked and
  explains the fix (`PYO3_PYTHON=...`), and the pymwemu Makefile and
  `test_all.sh` prefer the newest `python3.x` on PATH when creating the venv.
  `pyproject.toml` now declares `requires-python = ">=3.10"` to match, and
  its `version` is bumped from the stale `0.11.6` to `1.0.0` like the crate.
- **`xlatb`** and **`das`** are implemented (both raised #UD).
- **Shellcode-based x86 tests** that need no sample binaries:
  `tests/isa/x64/shellcode_{arith,bits,control,memory}.rs` and
  `tests/isa/x86/shellcode_x86.rs` run small hand-assembled programs
  (arithmetic and flag boundaries, shifts/rotates, every `jcc`/`cmovcc`,
  calls and the stack, string instructions, CAS loops, a bubble sort...) and
  `tests/isa/x64/cpu_vectors.rs` replays 344 register/memory vectors recorded
  from the real CPU (`scripts/rosetta-difftest/gen_vectors.py`).
- **Rosetta differential harness** (`scripts/rosetta-difftest/`): on Apple
  Silicon, runs random x86-64 snippets on the real CPU through Rosetta and in
  pymwemu and diffs registers, memory and the defined flags. It found the
  `imul`/`div`/`loop`/`jecxz`/`scas`/`bt*`/REP bugs above; regression tests
  live in `tests/isa/x64/rosetta_regressions.rs`.
- **Deterministic runs**: `Config::seed` / `Emu::set_seed(seed)`, the CLI
  `--seed <N>` flag and the pymwemu `seed` property seed the emulator's random
  source so `rdrand`/`rdseed` return the same sequence on every run.
  `Emu::random_u64()` exposes the source.

## [1.0.0] - 2026-10-24

First stable release. All crates bumped to 1.0.0: libmwemu, mwemu, pymwemu,
cmwemu, mwemu-mcp, mwemu-snapshot, mwemu-x86-test.

### Added

- **Fuzz harness**: `cargo-fuzz` targets for `load_code` (PE/ELF/Mach-O) and
  `run` on random shellcode, with 30s CI smoke pass per target.
- **Config builder pattern**: 48 `set_*` methods returning `&mut Self` for
  chaining on the `Config` struct.
- **Hook delegation**: `set_hook_on_*` convenience methods on `Emu` so users
  don't need to access `emu.hooks` directly. `clear_hooks()` added.
- **Core Windows API stubs**: ole32 (`CoInitialize`, `CoCreateInstance`,
  `CoTaskMemAlloc/Free`), advapi32 (`RegOpenKeyExA/W`, `RegQueryValueExA/W`,
  `RegCloseKey`), crypt32 (`CryptDecodeObjectEx`, `CertOpenStore`,
  `CertOpenSystemStoreA/W`, `CertCloseStore`, `CertFindCertificateInStore`,
  `CertFreeCertificateContext`), user32 (`wsprintfA/W`, `LoadStringA/W`,
  `CharNextA`), kernel32 (`CreateFileMappingA/W`, `MapViewOfFile`), ntdll
  (`RtlGetVersion`, `NtQueryInformationProcess`).
- **32-bit parity**: ntdll cross-module call interception for 32-bit,
  `_HEAP` structure emulation for 32-bit, `VirtualAllocEx` 32-bit fix.
- **Syscall mode robustness**: TEB fields (`+0x1480` FlsData, `+0x1858`
  heap-related), `NtQueryVirtualMemory`, `NtProtectVirtualMemory`,
  `NtAllocateVirtualMemory` stubs.
- **CI improvements**: panic-gate (`scripts/panic-gate.sh`), integration
  tests in matrix (Linux/Windows/macOS), clippy `-D warnings`.
- **Doc comments** (`///`) on all public API types and methods.
- **Examples**: 16 pymwemu scripts, 3 Jupyter notebooks, cmwemu `demo.c`.

### Changed

- `load_code` and `load_code_bytes` now return `Result<(), MwemuError>`
  instead of panicking on malformed input.
- `run`, `run_to`, `run_until_ret` return `Result<u64, MwemuError>`.
- `alloc()` returns `Option<u64>`, `create_map()` returns `Result`.
- Restricted `pub` visibility: 7 modules to `pub(crate)`, ~50 Emu struct
  fields to `pub(crate)`, `InstructionState` to `pub(crate)`.
- Unimplemented x86 instructions now raise `#UD` (IllegalInstruction
  exception via SEH/VEH) instead of silent stop.
- MSRV bumped to 1.95.

### Fixed

- **Zero guest-reachable panics**: all 98 `unimplemented!()` converted to
  log + graceful stop; all guest-reachable `unwrap()`/`expect()` replaced
  with safe defaults or error returns; `unreachable!()` in FPU/instructions
  replaced with log + safe return.
- Arithmetic bug in FPU instructions (fuzz-discovered).
- 1-operand memory form handling in FPU instructions (fuzz crash fix).

### Removed

- Grandfathered clippy allow blocks (entire burndown backlog eliminated).
- Unused `pub use` re-exports in structures and winapi modules.
