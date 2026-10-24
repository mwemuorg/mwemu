# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
