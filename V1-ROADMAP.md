# libmwemu v1.0 Roadmap — COMPLETE

All P0 and P1 items are done (P1 #7 BlocksIndex deferred — low urgency).
P2 #8 (Documentation) complete. P2 #9 (CI) complete. P2 #10 (Performance)
deferred as nice-to-have. Version 1.0.0 released.

Current state: 733+ tests, multi-platform (PE/ELF/Mach-O), x86/x64/AArch64,
kernel-mode driver emulation, syscall mode, and Windows system simulation
(PEB/TEB/LDR, heap, TLS, IAT binding, SEH/VEH). CI runs `fmt`, `clippy`
(`-D warnings`), `test` matrix, panic-gate, and fuzz smoke.

> Counts re-audited on 2026-10-21 against `crates/libmwemu/src` (v0.26.0).
> The previous audit (2026-09-28) quoted 11 `unimplemented!()` — the real
> count was ~98 (most with message strings that the grep missed). All 98
> have now been eliminated.

---

## P0 — Must-have (stability contract)

### 1. No panics on guest code

The emulator must never abort on any guest binary, no matter how malformed.
The real crash surface is not the handful of `unimplemented!()` macros — it
is the mass of `unwrap()`/`expect()` on paths reachable from guest bytes.

Current audit (v0.26.0 → v0.26.3):

| Signal                | v0.26.0 | v0.26.1  | v0.26.2  | v0.26.3  | Guest-reachable hot spots            |
|-----------------------|---------|----------|----------|----------|--------------------------------------|
| `unimplemented!()`    | 98      | **0**    | **0**    | **0**    | All converted to log + graceful stop (v0.26.0) |
| `unreachable!()`      | —       | —        | 11       | **0** ✅ | 57 remain but all are arch-mismatch host invariants |
| `panic!(...)`         | 15      | 15       | **15**   | **6**    | 4 arch-assertion guards, 1 heap init fatal, 1 kuser init |
| `.unwrap()`           | 580     | 796†     | **447**  | **126**  | 0 guest-reachable; rest are Mutex::lock, try_into on fixed slices, serialization I/O, tests |
| `.expect(...)`        | 2896    | 480      | **312**  | **51**   | **0 guest-reachable**; heap tests, maps host-invariant, rest test/Mutex/conditional |

† unwrap count rose in v0.26.1 because ~2500 `expect()` in api/ were converted
  to `unwrap_or(0)`. v0.26.2: structures/ 346 unwraps → `unwrap_or(0)`,
  api/syscall/init expect→unwrap_or conversions. v0.26.3: unreachable!() in
  FPU/instructions/ntoskrnl/exception_handlers → log + safe return; file I/O
  unwraps in create_file, ntdll/file, ws2_32, kernelbase, lock_resource,
  sizeof_resource → match + error return. Final sweep: abi.rs, libsystem.rs
  gateway, winapi gateways, f80.rs BCD, regs.rs, debug/script.rs,
  syscall32/misc.rs socket dispatch. **Zero guest-reachable panics remain.**

**Work:**
- [x] Replace all `unimplemented!()` with log + graceful stop. DLL gateways
      use `log::error`, ISA handlers use `log::warn + return false`, API
      stubs log and return defaults. Done in v0.26.0 (98 → 0).
- [x] **engine/instructions/**: all `rep.unwrap()` → `unwrap_or(0)`, all
      `stack_pop*.unwrap()` → match+return false, all memory read `.expect()`
      → match+return false (pop, fadd, ret, pshufd, movs*, cmps*, stos*, scas*).
- [x] **maps/**: `mem64.rs` slice/file I/O unwraps → `unwrap_or`/log+return,
      `mod.rs` map lookups, free/dealloc, alloc, cross-map writes → log+return.
      `search.rs` hex parse → graceful return.
- [x] **emu/operands.rs**: banzai map `.expect()` → match+return false.
- [x] **emu/instruction_pointer.rs**: `get_addr_name().expect()` → match+return false.
- [x] **emu/object_handle/hive_parser.rs**: `RegType` conversion → `?` propagation.
- [x] **emu/memory.rs**: all `unreachable!()` and `expect()` → log::warn + return 0.
- [x] **emu/disassemble**: `expect("address not mapped")` → return error string.
- [x] **emu/execution**: dump/trace file unwrap/expect → if-let-Some pattern.
- [x] **emu/trace.rs**: file creation expect → graceful error handling.
- [x] **emu/iso.rs, winver.rs**: `to_str().unwrap()` → guarded with if-let.
- [x] **emu/object_handle**: file_handle/windows_path unwrap → ok_or/ok_or_else.
- [x] **emu/loaders/pe.rs**: `expect("out of memory")` → match + return (0,0).
- [x] **emu/loaders.rs**: map creation, heap allocation expects → log + return.
- [x] **emu/loaders/elf.rs**: interp unwrap → unwrap_or_default.
- [x] **loaders/macho/macho64.rs**: `reparse().expect()` → match Ok/Err +
      return empty; bounds checks on all byte-slice indexing in chained fixups.
- [x] **api/ winapi stubs:** ~2500 `.expect()` on stack reads converted to
      `.unwrap_or(0)` (safe default — gateway sets up the stack). Remaining
      70 `.expect()` in api/ converted to match+log or unwrap_or (v0.26.2).
      Hold the line with a CI grep gate on `engine/`, `maps/`, `emu/` core
      (see P2 #9).
- [x] **syscall/**: 72 `.expect()` → `unwrap_or(0)` for reads, match+log
      for alloc/file ops (v0.26.2).
- [x] **structures/**: 346 `.unwrap()` → `unwrap_or(0)` on PEB/TEB/system_info
      memory reads (v0.26.2).
- [x] **initialization.rs**: 26 `.expect()` → Result propagation via `?`/match
      for map creation, heap alloc, banzai CSV parse (v0.26.2).
- [x] **loaders (pe_stub, ko, kext)**: 7 `.expect()` → Result propagation (v0.26.2).
- [x] Unimplemented x86 instructions now raise `#UD` (IllegalInstruction
      exception via SEH/VEH) instead of `return false` + silent stop.

### 2. Public API stability

The `libmwemu` crate API is what downstream users (pymwemu, cmwemu, custom
tools) depend on. A v1 promises it won't break without a major bump.

**Work:**
- [x] Review `pub` visibility — internals that leaked as `pub` should be
      `pub(crate)`. Done: 7 modules in lib.rs restricted to `pub(crate)`
      (api, engine, exception, loaders, syscall, threading, windows);
      4 internal re-exports to `pub(crate) use`; ~50 Emu struct fields
      restricted to `pub(crate)` (only 20 fields remain `pub`: those used
      by pymwemu, cmwemu, CLI, MCP, snapshot, and x86-test).
      `InstructionState` struct made `pub(crate)`. All workspace crates
      compile, 735 tests pass.
- [x] Stable entry points: `Emu::new`, `load_code`, `run`, `run_to`,
      `set_hook`, config fields. Documented with `///` doc comments on all
      public API methods (Config, Emu, Hooks, Maps, stack, trace, execution).
- [x] Stabilize the `Hooks` callback signatures. Done: hook convenience
      methods added to `Emu` (`set_hook_on_*`, `clear_hooks`) so users
      don't access `emu.hooks` directly. Signatures stable since
      `DecodedInstruction` migration.
- [x] `cfg` fields that control behavior should have builder methods or
      a `Config` struct with `Default`. Done: 48 `set_*` builder methods
      on `Config` returning `&mut Self` for chaining. `Default` impl
      already existed.

### 3. Error handling

The public signatures are already `Result`-based (`run`, `run_to`,
`run_until_ret`, and the per-arch variants all return
`Result<u64, MwemuError>`). The open work is making the internals honor
that contract instead of panicking — which is the same sweep as P0 #1.

- [x] `run()` must propagate every internal failure as `Err`, never abort.
      Zero guest-reachable panics remain (v0.26.3). Remaining `panic!` are
      4 arch-assertion API guards and 1 heap-init fatal (host invariant).
- [x] Malformed PE/ELF/Mach-O input: return `Err` from `load_code`, not
      panic on short reads or missing sections. `load_code` and
      `load_code_bytes` now return `Result<(), MwemuError>`. All callers
      (CLI, pymwemu, cmwemu, MCP, examples, tests) updated.
- [x] OOM in `maps.alloc()` / `create_map()`: `alloc()` returns `Option<u64>`,
      `create_map()` returns `Result<&mut Mem64, String>`. All callers
      handle `None`/`Err` gracefully. No `.expect()` on alloc results
      outside test code.

### 4. Fuzz the input surface (new)

P0 is "don't crash on malformed input"; the only way to *prove* it is to
feed random bytes at the entry points. This also operationalizes the
Enigma-canary philosophy the rest of the suite already follows.

- [x] `cargo-fuzz` harness over `load_code` (PE/ELF/Mach-O) and
      over `run` on random shellcode buffers (`fuzz/fuzz_targets/`).
- [x] Run a short fuzz smoke pass in CI (30s per target, both x86 and x64).
      Corpus of crashers kept as regression seeds.

---

## P1 — Should-have (completeness)

### 5. Core Windows API coverage

Triage the top unimplemented APIs hit by real samples and implement stubs.
Priority list (based on Enigma, Themida, and common malware):

- [x] **ole32**: `CoInitialize`, `CoCreateInstance`, `CoTaskMemAlloc/Free`
- [x] **advapi32**: `RegOpenKeyExA/W`, `RegQueryValueExA/W`, `RegCloseKey`
- [x] **crypt32**: `CryptDecodeObjectEx`, `CertOpenStore`, `CertOpenSystemStoreA/W`,
      `CertCloseStore`, `CertFindCertificateInStore`, `CertFreeCertificateContext`
      (32-bit and 64-bit stubs)
- [x] **user32**: `wsprintfA/W`, `LoadStringA/W`, `CharNextA`
- [x] **kernel32**: `CreateFileMappingA/W`, `MapViewOfFile` (basic)
- [x] **ntdll**: `RtlGetVersion`, `NtQueryInformationProcess` (anti-debug)

### 6. 32-bit / 64-bit parity

- [x] ntdll cross-module call interception (done for 64-bit, needs 32-bit)
      Added `is_ntdll_cross_call` detection in `set_eip` mirroring the 64-bit
      `set_rip` logic. Cross-module calls into ntdll32 are now intercepted by
      the gateway instead of executing raw bytes.
- [x] `_HEAP` structure emulation for 32-bit
      New `heap32.rs` models the Win10 x86 `_HEAP` (SegmentSignature, Flags,
      SegmentList, BlocksIndex, FreeLists, LockVariable). `peb32::init_peb()`
      now allocates and writes the ProcessHeap at PEB32 offset +0x18.
- [x] VirtualAllocEx for 32-bit (same commit-without-reserve fix)
- [x] Test coverage: 32-bit mirrors for GetModuleHandleA, CloseHandle added.
      9 64-bit tests without mirrors audited; remaining are COM/WinHTTP/user32
      stubs not yet implemented in 32-bit.

### 7. Syscall mode (`--syscall-mode`) robustness

- [ ] `_HEAP` struct: populate `BlocksIndex` function pointers or intercept
      CFG dispatch so ntdll heap code doesn't crash (see `docs/ANTI_EMU_HEAP.md`).
      Low urgency: `VirtualMemoryThreshold = 0xFE00` routes most allocations
      away from BlocksIndex; only large-block paths (>0xFE00) are affected.
- [x] TEB fields: `+0x1480` (FlsData), `+0x1858` (heap-related) used by
      ntdll internals. `ensure_teb_fls_and_heap_fields()` allocates a zeroed
      FLS region and writes `&_HEAP.Flags` so ntdll reads don't crash.
- [x] More syscall stubs: `NtQueryVirtualMemory`, `NtProtectVirtualMemory`,
      `NtAllocateVirtualMemory` — all three already implemented in
      `syscall/windows/syscall64/memory.rs`.

---

## P2 — Nice-to-have (polish)

### 8. Documentation

- [x] `///` doc comments on all public API types and methods (Emu, Config,
      MwemuError, Hooks, Maps, Mem64, Permission, Arch, OperatingSystem, register
      accessors, emu32/emu64/emu_aarch64, load_code, run/step/run_to/run_until_ret,
      Breakpoint, Breakpoints, stack, trace, call_stack, execution, debug)
- [x] `examples/` directory (01_shellcode, 02_memory, 03_hooks,
      04_load_binary already exist)
- [x] Add examples for the Python (pymwemu) and C (cmwemu) bindings
      pymwemu: 16 scripts in `examples/scripts/` + 3 Jupyter notebooks.
      cmwemu: `examples/demo.c` with build instructions.
- [x] `CHANGELOG.md`

### 9. CI improvements

CI already has `fmt`, `clippy`, and a `test` matrix. What's missing:

- [x] Make clippy **fail on warnings** (`-D warnings`) — done in CI and
      `make clippy`. MSRV was bumped 1.88 -> 1.95 so the auto-fixes (which use
      `is_multiple_of()` etc.) are valid. ~613 -> the residual backlog is
      grandfathered via crate-level `#![allow(...)]` blocks marked
      "clippy v1 burn-down backlog"; new warnings outside that set now fail.
- [x] **Burn down** the grandfathered clippy allow list. **COMPLETE.** The
      entire burndown backlog block removed from `lib.rs`. All 27 grandfathered
      allows eliminated: 5 zero-warning removals, 12 source-code fixes in round
      one, 10 more source fixes + local allows in round two (46 warnings total).
      Only two deliberate crate-level allows remain: `neg_cmp_op_on_partial_ord`
      (NaN semantics in float compare handlers) and `manual_div_ceil` (external
      `construct_uint!` macro expansion).
- [x] Add the integration tests (test_linux, test_windows, test_syscall) to
      the CI matrix. Done: CI runs on Linux, Windows, macOS with
      platform-filtered nextest expressions.
- [x] Add the P0 grep gate: fail if `unwrap`/`expect`/`panic!` appears on
      guest-reachable paths (`emu/`, `engine/`, `maps/`), and a panic-count
      regression check so the number can only go down. Done:
      `scripts/panic-gate.sh` runs in CI as a separate job.

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
