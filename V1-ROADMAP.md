# libmwemu v1.0 Roadmap

What needs to happen before cutting a stable 1.0 release.

Current state: 560+ tests, multi-platform (PE/ELF/Mach-O), x86/x64/AArch64,
kernel-mode driver emulation, syscall mode, and Windows system simulation
(PEB/TEB/LDR, heap, TLS, IAT binding, SEH/VEH). CI already runs `fmt`,
`clippy`, and a `test` matrix; `examples/` (01–04) exists.

> Counts below were re-audited on 2026-09-28 against `crates/libmwemu/src`.
> Re-run the audit before turning any P0 item into a GitHub issue — the
> previous version of this file quoted stale numbers (e.g. "48
> `unimplemented!()` across DLL gateways", now 11 and mostly not in gateways).

---

## P0 — Must-have (stability contract)

### 1. No panics on guest code

The emulator must never abort on any guest binary, no matter how malformed.
The real crash surface is not the handful of `unimplemented!()` macros — it
is the mass of `unwrap()`/`expect()` on paths reachable from guest bytes.

Current audit:

| Signal                | Total | Guest-reachable hot spots            |
|-----------------------|-------|--------------------------------------|
| `unimplemented!()`    | 11    | SSE handlers (`psubb/w/d/q`, `movhpd`), `arch/x86/regs.rs`, `emu/memory.rs`, `mscoree`, `ntapi32` |
| `panic!(...)`         | 13    | `emu/`, `engine/`, `maps/`           |
| `.unwrap()`           | ~811  | emu 57, engine 24, maps 54           |
| `.expect(...)`        | ~3032 | emu 63, engine 8, maps 32            |

**Work:**
- [ ] **Headline gate:** no `unwrap`/`expect`/`panic!` on any path reachable
      from guest bytes (start with `emu/`, `engine/`, `maps/`). Convert to
      `Result` or a guest exception. Hold the line with a CI grep gate on
      those modules (see P2 #8).
- [ ] Replace the 11 `unimplemented!()` with a log + skip (or return-error
      when `skip_unimplemented` is false). Note: only `mscoree.rs` and
      `ntapi32.rs` are API/syscall gateways now; the rest are ISA handlers
      and should instead raise `#UD` — fold into the next bullet.
- [ ] Unimplemented x86 instructions should raise `#UD` (exception), not
      `return false` + silent stop.

### 2. Public API stability

The `libmwemu` crate API is what downstream users (pymwemu, cmwemu, custom
tools) depend on. A v1 promises it won't break without a major bump.

**Work:**
- [ ] Review `pub` visibility — internals that leaked as `pub` should be
      `pub(crate)`.
- [ ] Stable entry points: `Emu::new`, `load_code`, `run`, `run_to`,
      `set_hook`, config fields. Document with `///` doc comments.
- [ ] Stabilize the `Hooks` callback signatures.
- [ ] `cfg` fields that control behavior should have builder methods or
      a `Config` struct with `Default`.

### 3. Error handling

The public signatures are already `Result`-based (`run`, `run_to`,
`run_until_ret`, and the per-arch variants all return
`Result<u64, MwemuError>`). The open work is making the internals honor
that contract instead of panicking — which is the same sweep as P0 #1.

- [ ] `run()` must propagate every internal failure as `Err`, never abort.
- [ ] Malformed PE/ELF/Mach-O input: return `Err` from `load_code`, not
      panic on short reads or missing sections.
- [ ] OOM in `maps.alloc()` / `create_map()`: propagate, don't `expect`.

### 4. Fuzz the input surface (new)

P0 is "don't crash on malformed input"; the only way to *prove* it is to
feed random bytes at the entry points. This also operationalizes the
Enigma-canary philosophy the rest of the suite already follows.

- [ ] `cargo-fuzz` (or `afl`) harness over `load_code` (PE/ELF/Mach-O) and
      over `run` on random code buffers.
- [ ] Run a short fuzz smoke pass in CI; keep a corpus of crashers as
      regression seeds.

---

## P1 — Should-have (completeness)

### 5. Core Windows API coverage

Triage the top unimplemented APIs hit by real samples and implement stubs.
Priority list (based on Enigma, Themida, and common malware):

- [ ] **ole32**: `CoInitialize`, `CoCreateInstance`, `CoTaskMemAlloc/Free`
- [ ] **advapi32**: `RegOpenKeyExA/W`, `RegQueryValueExA/W`, `RegCloseKey`
- [ ] **crypt32**: `CryptDecodeObjectEx`, `CertOpenStore` (stub)
- [ ] **user32**: `wsprintfA/W`, `LoadStringA/W`, `CharNextA`
- [ ] **kernel32**: `CreateFileMappingA/W`, `MapViewOfFile` (basic)
- [ ] **ntdll**: `RtlGetVersion`, `NtQueryInformationProcess` (anti-debug)

### 6. 32-bit / 64-bit parity

- [ ] ntdll cross-module call interception (done for 64-bit, needs 32-bit)
- [ ] `_HEAP` structure emulation for 32-bit
- [ ] VirtualAllocEx for 32-bit (same commit-without-reserve fix)
- [ ] Test coverage: every 64-bit winapi test should have a 32-bit mirror

### 7. Syscall mode (`--syscall-mode`) robustness

- [ ] `_HEAP` struct: populate `BlocksIndex` function pointers or intercept
      CFG dispatch so ntdll heap code doesn't crash (see `docs/ANTI_EMU_HEAP.md`)
- [ ] TEB fields: `+0x1480` (FlsData), `+0x1858` (heap-related) used by
      ntdll internals
- [ ] More syscall stubs: `NtQueryVirtualMemory`, `NtProtectVirtualMemory`,
      `NtAllocateVirtualMemory`

---

## P2 — Nice-to-have (polish)

### 8. Documentation

- [ ] `///` doc comments on all public API types and methods
- [x] `examples/` directory (01_shellcode, 02_memory, 03_hooks,
      04_load_binary already exist)
- [ ] Add examples for the Python (pymwemu) and C (cmwemu) bindings
- [ ] `CHANGELOG.md`

### 9. CI improvements

CI already has `fmt`, `clippy`, and a `test` matrix. What's missing:

- [x] Make clippy **fail on warnings** (`-D warnings`) — done in CI and
      `make clippy`. MSRV was bumped 1.88 -> 1.95 so the auto-fixes (which use
      `is_multiple_of()` etc.) are valid. ~613 -> the residual backlog is
      grandfathered via crate-level `#![allow(...)]` blocks marked
      "clippy v1 burn-down backlog"; new warnings outside that set now fail.
- [ ] **Burn down** the grandfathered clippy allow list (search the codebase
      for "clippy v1 burn-down backlog") and remove entries as fixed. Keep
      `neg_cmp_op_on_partial_ord` — it guards intended NaN semantics in the
      float-compare handlers.
- [ ] Add the integration tests (test_linux, test_windows, test_syscall) to
      the CI matrix.
- [ ] Add the P0 grep gate: fail if `unwrap`/`expect`/`panic!` appears on
      guest-reachable paths (`emu/`, `engine/`, `maps/`), and a panic-count
      regression check so the number can only go down.

### 10. Performance

- [ ] Profile hot paths (instruction decode + dispatch, memory read/write)
- [ ] `get_addr_name()` has ~57 call sites and runs on RIP changes — confirm
      it's still on the hot path, then cache the lookups if so
- [ ] Benchmark: instructions-per-second on reference samples

---

## Non-goals for v1

- Full Windows kernel emulation (kernel-mode is experimental, driver-focused)
- GUI / debugger frontend (the console debugger `-c` is enough)
- Network I/O emulation (ws2_32 stubs are sufficient)
- Multi-threaded guest execution (single-threaded scheduler is fine)

---

## How to track progress

Each item above can be a GitHub issue. The TEST.md integration tests are the
gate: nothing ships if any step fails. The Enigma binary
(`test/exe64win_enigma.bin`) is the canary — it exercises heap, TLS, API
resolution, and anti-emulation checks harder than anything else in the test
suite.
