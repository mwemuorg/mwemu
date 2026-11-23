use crate::tests::helpers;
use crate::tests::loaders::{HELLO_WORLD_STEPS, write_hello_fixture};
use crate::*;

const HELLO_WIN_X86: &[u8] = include_bytes!("../../fixtures/hello_win_x86.exe");
const HELLO_WIN_X64: &[u8] = include_bytes!("../../fixtures/hello_win_x64.exe");
const HELLO_WIN_ARM64: &[u8] = include_bytes!("../../fixtures/hello_win_arm64.exe");

/// Windows PE32 x86 hello world — loads and detects correct arch.
#[cfg(feature = "asset-tests")]
#[test]
fn hello_win_x86() {
    helpers::setup();
    helpers::fixture_maps32();
    let path = write_hello_fixture("mwemu_hello_win_x86.exe", HELLO_WIN_X86);

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

        for _ in 0..HELLO_WORLD_STEPS {
            if !emu.step() {
                break;
            }
        }
    })
    .expect("hello_win_x86 exceeded wall-clock budget");
}

/// Windows x86_64 PE hello world — loads and detects correct arch.
#[cfg(feature = "asset-tests")]
#[test]
fn hello_win_x64() {
    helpers::setup();
    helpers::fixture_maps64();
    let path = write_hello_fixture("mwemu_hello_win_x64.exe", HELLO_WIN_X64);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu64();
        emu.load_code(path.to_str().unwrap());

        assert!(emu.cfg.arch.is_x64(), "expected PE64 x86_64 dispatch");
        let entry = emu.regs().rip;
        assert!(entry != 0, "entry point should be set");

        for _ in 0..HELLO_WORLD_STEPS {
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
///      Reaching the asserts without a panic/fatal fault proves both.
#[cfg(feature = "asset-tests")]
#[test]
fn hello_win_x64_runs_tls_callbacks() {
    helpers::setup();
    helpers::fixture_maps64();
    let path = write_hello_fixture("mwemu_hello_win_x64_tls.exe", HELLO_WIN_X64);

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
    let path = write_hello_fixture("mwemu_hello_win_arm64.exe", HELLO_WIN_ARM64);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu_aarch64();
        emu.load_code(path.to_str().unwrap())
            .expect("load the Windows ARM64 fixture");

        assert!(
            emu.cfg.arch.is_aarch64(),
            "expected PE aarch64 dispatch, got {:?}",
            emu.cfg.arch
        );
        let pc = emu.pc();
        assert!(pc != 0, "entry point should be set");

        for _ in 0..HELLO_WORLD_STEPS {
            if !emu.step() {
                break;
            }
        }
    })
    .expect("hello_win_arm64 exceeded wall-clock budget");
}
