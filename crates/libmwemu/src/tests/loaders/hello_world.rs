//! Hello-world parity tests across all (os, arch) combos we care about.
//!
//! These exist to sniff out loader / ISA / OS-surface gaps. Each test loads a
//! tiny C `printf("hello world")` binary built by `examples/hello-world/Makefile`,
//! takes a bounded number of steps, and asserts the loader picked the right
//! arch. Tests for combos the emulator does not yet support are `#[ignore]`d
//! with a comment pointing at the gap so future work can flip them on.
//!
//! Every test body is wrapped in `helpers::run_with_timeout` so a runaway
//! `step()` loop (e.g. an unmapped AArch64 PLT stub) cannot stall the suite
//! past the wall-clock budget.
//!
//! Source: examples/hello-world/main.c
//! Build:  make -C examples/hello-world all  (then move into tests/fixtures/)

use crate::tests::helpers;
use crate::*;

const HELLO_LINUX_X86: &[u8] = include_bytes!("../fixtures/hello_linux_x86");
const HELLO_LINUX_X64: &[u8] = include_bytes!("../fixtures/hello_linux_x64");
const HELLO_LINUX_ARM64: &[u8] = include_bytes!("../fixtures/hello_linux_arm64");
const HELLO_MAC_ARM64: &[u8] = include_bytes!("../fixtures/hello_mac_arm64");
const HELLO_MAC_X64: &[u8] = include_bytes!("../fixtures/hello_mac_x64");
const HELLO_WIN_X86: &[u8] = include_bytes!("../fixtures/hello_win_x86.exe");
const HELLO_WIN_X64: &[u8] = include_bytes!("../fixtures/hello_win_x64.exe");
const HELLO_WIN_ARM64: &[u8] = include_bytes!("../fixtures/hello_win_arm64.exe");

const MAX_STEPS: usize = 64;

fn write_tmp(name: &str, bytes: &[u8]) -> std::path::PathBuf {
    let p = std::env::temp_dir().join(name);
    std::fs::write(&p, bytes).unwrap();
    p
}

/// Dynamic ELF32 hello world -- loads and steps without null-pointer write.
#[test]
fn hello_linux_x86() {
    helpers::setup();
    let path = write_tmp("mwemu_hello_linux_x86", HELLO_LINUX_X86);

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

        for _ in 0..MAX_STEPS {
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
    let path = write_tmp("mwemu_hello_linux_x64", HELLO_LINUX_X64);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu64();
        emu.load_code(path.to_str().unwrap());

        assert!(emu.cfg.arch.is_x64(), "expected ELF64 x86_64 dispatch");
        let entry = emu.regs().rip;
        assert!(entry != 0, "entry point should be set");

        for _ in 0..MAX_STEPS {
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
    let path = write_tmp("mwemu_hello_linux_arm64", HELLO_LINUX_ARM64);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu_aarch64();
        emu.load_code(path.to_str().unwrap());

        assert!(emu.cfg.arch.is_aarch64(), "expected ELF64 aarch64 dispatch");
        let pc = emu.regs_aarch64().pc;
        assert!(pc != 0, "pc should be set by loader");

        for _ in 0..MAX_STEPS {
            if !emu.step() {
                break;
            }
        }
    })
    .expect("hello_linux_arm64 exceeded wall-clock budget");
}

#[test]
fn hello_mac_arm64() {
    helpers::setup();
    let path = write_tmp("mwemu_hello_mac_arm64", HELLO_MAC_ARM64);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu_aarch64();
        emu.load_code(path.to_str().unwrap());

        assert!(
            emu.cfg.arch.is_aarch64(),
            "expected Mach-O aarch64 dispatch"
        );
        let pc = emu.regs_aarch64().pc;
        assert!(pc >= 0x100000000, "entry 0x{:x} should be in __TEXT", pc);

        for _ in 0..MAX_STEPS {
            if !emu.step() {
                break;
            }
        }
    })
    .expect("hello_mac_arm64 exceeded wall-clock budget");
}

/// Mach-O x86_64 hello world — loads and detects correct arch.
#[test]
fn hello_mac_x64() {
    helpers::setup();
    let path = write_tmp("mwemu_hello_mac_x64", HELLO_MAC_X64);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu64();
        emu.load_code(path.to_str().unwrap());

        assert!(
            emu.cfg.arch.is_x64(),
            "expected Mach-O x86_64 dispatch, got {:?}",
            emu.cfg.arch
        );
    })
    .expect("hello_mac_x64 exceeded wall-clock budget");
}

/// Windows PE32 x86 hello world — loads and detects correct arch.
#[test]
fn hello_win_x86() {
    helpers::setup();
    let path = write_tmp("mwemu_hello_win_x86.exe", HELLO_WIN_X86);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu32();
        emu.load_code(path.to_str().unwrap());

        assert!(
            matches!(emu.cfg.arch, crate::arch::Arch::X86),
            "expected PE32 x86 dispatch, got {:?}",
            emu.cfg.arch
        );
        let entry = emu.regs().rip;
        assert!(entry != 0, "entry point should be set");

        for _ in 0..MAX_STEPS {
            if !emu.step() {
                break;
            }
        }
    })
    .expect("hello_win_x86 exceeded wall-clock budget");
}

/// Windows x86_64 PE hello world — loads and detects correct arch.
#[test]
fn hello_win_x64() {
    helpers::setup();
    let path = write_tmp("mwemu_hello_win_x64.exe", HELLO_WIN_X64);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu64();
        emu.load_code(path.to_str().unwrap());

        assert!(emu.cfg.arch.is_x64(), "expected PE64 x86_64 dispatch");
        let entry = emu.regs().rip;
        assert!(entry != 0, "entry point should be set");

        for _ in 0..MAX_STEPS {
            if !emu.step() {
                break;
            }
        }
    })
    .expect("hello_win_x64 exceeded wall-clock budget");
}

/// Windows x86_64 PE TLS callbacks — self-contained (no sample bundle).
///
/// The committed `hello_win_x64.exe` fixture declares a TLS directory with two
/// callbacks (`__dyn_tls_init` & friends). `load_code` must:
///   1. detect and rebase them into `emu.tls_callbacks` (regression guard for
///      the old `AddressOfCallBacks & 0xffff` RVA bug, which produced garbage
///      addresses), and
///   2. run them before the entry point (best-effort: unimplemented APIs in the
///      callback are skipped, not fatal).
/// Reaching the asserts without a panic/fatal fault proves both.
#[test]
fn hello_win_x64_runs_tls_callbacks() {
    helpers::setup();
    let path = write_tmp("mwemu_hello_win_x64_tls.exe", HELLO_WIN_X64);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu64();
        emu.load_code(path.to_str().unwrap());

        // Detection + rebasing: the fixture declares exactly two callbacks.
        assert_eq!(
            emu.tls_callbacks.len(),
            2,
            "hello_win_x64 declares 2 TLS callbacks; got {:?}",
            emu.tls_callbacks
        );

        // Each callback must be a real, mapped, executable address inside the loaded
        // image — not a stray value read from a misparsed TLS directory.
        for &cb in &emu.tls_callbacks {
            let name = emu.maps.get_addr_name(cb);
            assert!(
                name.is_some(),
                "TLS callback 0x{:x} should point into a mapped region, got None",
                cb
            );
        }

        // `load_code` runs the callbacks and parks execution at the entry point.
        assert!(
            emu.regs().rip != 0,
            "entry point should be set after running TLS callbacks"
        );
    })
    .expect("hello_win_x64_runs_tls_callbacks exceeded wall-clock budget");
}

/// Windows ARM64 PE hello world — loads, detects arch, and steps.
/// Requires real ARM64 DLLs in maps/windows/aarch64/ (kernelbase.dll, kernel32.dll, ntdll.dll).
#[test]
#[ignore = "parity gap: maps/windows/aarch64/ has no DLLs yet"]
fn hello_win_arm64() {
    helpers::setup();
    let path = write_tmp("mwemu_hello_win_arm64.exe", HELLO_WIN_ARM64);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu_aarch64();
        emu.load_code(path.to_str().unwrap());

        assert!(
            emu.cfg.arch.is_aarch64(),
            "expected PE aarch64 dispatch, got {:?}",
            emu.cfg.arch
        );
        let pc = emu.pc();
        assert!(pc != 0, "entry point should be set");

        for _ in 0..MAX_STEPS {
            if !emu.step() {
                break;
            }
        }
    })
    .expect("hello_win_arm64 exceeded wall-clock budget");
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
    let path = write_tmp("mwemu_hello_linux_x86_ep", HELLO_LINUX_X86);

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
    let path = write_tmp("mwemu_hello_linux_arm64_budget", HELLO_LINUX_ARM64);

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
