# Serialization Roadmap

Current status:

- Native `dump_to_file` / `load_from_file` works for x86/x86_64/AArch64 with dedicated roundtrip coverage.
- Minidump `dump_to_minidump` / `load_from_minidump` works for x86/x86_64/AArch64.
- All previously-ignored AArch64 serialization and minidump tests are enabled and passing.

## Native snapshot compatibility

The writer uses one development format named `UNSTABLE`. Keep the build or source revision that wrote a snapshot if you need to resume it later. The layout can change between commits under the same header. The reader also accepts the frozen formats V1–V5.

Add guest state to the existing unstable layout and cover it with a round-trip test. Introduce a numbered format only when a consumer needs a compatibility guarantee across builds. At that point, freeze the complete wire layout, including its nested types, reserve a fresh numeric identifier, and keep a read test using data from the build that wrote it. Continue development under `UNSTABLE`.

## Phase 1: Native Serialization Architecture Parity

- [x] Replace x86-only `SerializableThreadContext` payloads with architecture-aware variants.
- [x] Add AArch64 thread serialization for `RegsAarch64`, pre-op regs, and post-op regs.
- [x] Refactor `SerializableEmu` flattened current-thread state so it is not hard-wired to x86 registers/flags/FPU.
- [x] Refactor instruction/decode serialization so `ArchState::X86` and `ArchState::AArch64` can both be represented and restored.
- [x] Make `SerializableEmu::from(&Emu)` stop using x86-only accessors on AArch64.
- [x] Make `From<SerializableEmu> for Emu` restore AArch64 thread state and `arch_state` structurally.
- [x] Investigate and fix the current stack overflow in `test_aarch64_native_serialization_fixture_roundtrip`.
- [x] Replace the placeholder serialization smoke test with real native roundtrip coverage.
- [x] Unignore `test_aarch64_native_serialization_fixture_roundtrip`.

## Phase 2: AArch64 Minidump Import

- [x] Parse `MinidumpRawContext::Arm64` and `MinidumpRawContext::OldArm64`.
- [x] Map ARM64 minidump register state into `RegsAarch64` (`x0..x30`, `sp`, `pc`, `nzcv`, FP/SIMD state).
- [x] Build an AArch64 `SerializableEmu` from imported minidumps instead of routing through x86-shaped fields.
- [x] Preserve AArch64 `cfg.arch`, OS, memory maps, modules, and current thread state on import.
- [x] Treat ARM64 PE modules as 64-bit when reconstructing imported module metadata.
- [x] Add an ARM64 minidump import fixture test.
- [x] Unignore `test_aarch64_minidump_fixture_roundtrip` once import/export are ready.

## Phase 3: AArch64 Minidump Export

- [x] Emit a valid `CONTEXT_ARM64` blob from `RegsAarch64`.
- [x] Teach the minidump writer to export `ArchThreadState::AArch64`.
- [x] Export ARM64 thread context, stack location, and relevant SIMD/FP register state.
- [x] Validate ARM64 minidumps with the Rust `minidump` parser.
- [ ] Validate generated dumps with at least one external consumer such as Ghidra or WinDbg.

## Done Means

- [x] Native serialization round-trips x86, x86_64, and AArch64 fixtures.
- [x] Minidump import/export round-trips x86, x86_64, and AArch64 fixtures.
- [x] The ignored AArch64 serialization/minidump tests are enabled and passing.
- [x] The old x86-only "too complex" placeholder test is removed or replaced with real assertions.

## Remaining

- [ ] Validate generated ARM64 dumps with at least one external consumer (Ghidra or WinDbg).
- [x] Refactor `Emu::new()` to accept an `Arch` parameter to avoid the two-step init_cpu pattern.
