use crate::tests::helpers;
use crate::tests::loaders::{HELLO_WORLD_STEPS, write_hello_fixture};
use crate::*;

const HELLO_LINUX_X86: &[u8] = include_bytes!("../../fixtures/hello_linux_x86");
const HELLO_LINUX_X64: &[u8] = include_bytes!("../../fixtures/hello_linux_x64");
const HELLO_LINUX_ARM64: &[u8] = include_bytes!("../../fixtures/hello_linux_arm64");

/// Dynamic ELF32 hello world -- loads and steps without null-pointer write.
#[test]
fn hello_linux_x86() {
    helpers::setup();
    let path = write_hello_fixture("mwemu_hello_linux_x86", HELLO_LINUX_X86);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu32();
        emu.load_code(path.to_str().unwrap());

        assert!(
            matches!(emu.cfg.arch, crate::arch::Arch::X86),
            "expected ELF32 x86 dispatch, got {:?}",
            emu.cfg.arch
        );
        let entry = emu.regs().rip;
        assert!(entry != 0, "entry point should be set");

        for _ in 0..HELLO_WORLD_STEPS {
            if !emu.step() {
                break;
            }
        }
    })
    .expect("hello_linux_x86 exceeded wall-clock budget");
}

/// Dynamic ELF64 x86_64 hello world -- loads and steps with stack layout.
#[test]
fn hello_linux_x64() {
    helpers::setup();
    let path = write_hello_fixture("mwemu_hello_linux_x64", HELLO_LINUX_X64);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu64();
        emu.load_code(path.to_str().unwrap());

        assert!(emu.cfg.arch.is_x64(), "expected ELF64 x86_64 dispatch");
        let entry = emu.regs().rip;
        assert!(entry != 0, "entry point should be set");

        for _ in 0..HELLO_WORLD_STEPS {
            if !emu.step() {
                break;
            }
        }
    })
    .expect("hello_linux_x64 exceeded wall-clock budget");
}

/// Dynamic ELF64 AArch64 hello world — loads and steps correctly.
#[test]
fn hello_linux_arm64() {
    helpers::setup();
    let path = write_hello_fixture("mwemu_hello_linux_arm64", HELLO_LINUX_ARM64);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu_aarch64();
        emu.load_code(path.to_str().unwrap());

        assert!(emu.cfg.arch.is_aarch64(), "expected ELF64 aarch64 dispatch");
        let pc = emu.regs_aarch64().pc;
        assert!(pc != 0, "pc should be set by loader");

        for _ in 0..HELLO_WORLD_STEPS {
            if !emu.step() {
                break;
            }
        }
    })
    .expect("hello_linux_arm64 exceeded wall-clock budget");
}

/// Regression test for the ELF32 entry-point bug: prior to the fix at
/// `emu/loaders.rs:100-119`, `load_code` set `cfg.arch` and allocated a
/// stack but never wrote `e_entry` into `regs_mut().rip`, so every ELF32
/// load left `rip == 0`. This test loads the committed `hello_linux_x86`
/// fixture, asserts `rip != 0`, and asserts `rip` falls inside a mapped,
/// readable region — both previously false.
#[test]
fn hello_linux_x86_loads_entry_point() {
    helpers::setup();
    let path = write_hello_fixture("mwemu_hello_linux_x86_ep", HELLO_LINUX_X86);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu32();
        emu.load_code(path.to_str().unwrap());
        let entry = emu.regs().rip;
        assert!(
            entry != 0,
            "ELF32 loader must set rip from e_entry, got 0x{:x}",
            entry
        );
        let code = match emu.maps.get_mem_by_addr(entry) {
            Some(c) => c,
            None => panic!(
                "ELF32 entry 0x{:x} does not land in any mapped region",
                entry
            ),
        };
        assert!(
            code.can_read(),
            "ELF32 entry 0x{:x} lands in a non-readable region",
            entry
        );
    })
    .expect("hello_linux_x86_loads_entry_point exceeded wall-clock budget");
}

/// Regression test for the AArch64 `step()` hang: prior to the fix at
/// `emu/execution/mod.rs::step_isa`, `step()` did not honor
/// `cfg.max_instructions`. The committed `hello_linux_arm64` fixture is
/// dynamically linked against libraries not present in the host stub-resolver
/// map (`maps/linux/aarch64/` does not exist); the binary walks forever on
/// PLT trampolines, and `step()` had no internal budget so the only ceiling
/// was cargo's 60-s default. This test sets `cfg.max_instructions = 4096`,
/// runs via `step()` (the path the previous bug hit), and asserts
/// `instruction_count` stayed within the budget — both the fix in
/// `step_isa` and the fix in `read_bytes` are needed for this to pass.
#[test]
fn hello_linux_arm64_respects_step_budget() {
    helpers::setup();
    let path = write_hello_fixture("mwemu_hello_linux_arm64_budget", HELLO_LINUX_ARM64);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu_aarch64();
        // Cap the inner loop with the engine's per-step budget; this is the
        // field the `step_isa` patch uses to break the loop on the very first
        // call where the AArch64 PLT goes off the rails.
        emu.cfg.max_instructions = Some(4096);
        emu.load_code(path.to_str().unwrap());

        assert!(emu.cfg.arch.is_aarch64(), "expected ELF64 aarch64");
        let pc = emu.regs_aarch64().pc;
        assert!(pc != 0, "pc should be set by loader");

        let mut steps = 0u64;
        while emu.step() && steps < 10_000 {
            steps += 1;
        }
        // The engine's runtime limit must have fired before our manual
        // 10 000-step ceiling — otherwise `step()` is still ignoring the cap
        // the fix put in.
        assert!(
            emu.instruction_count <= 4096 + 32, // 32 = slack for housekeeping
            "step() did not honor max_instructions: ran {} instructions",
            emu.instruction_count
        );
    })
    .expect("hello_linux_arm64_respects_step_budget exceeded wall-clock budget");
}
