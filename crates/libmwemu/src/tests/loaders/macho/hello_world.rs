use crate::tests::helpers;
use crate::tests::loaders::{HELLO_WORLD_STEPS, write_hello_fixture};
use crate::*;

const HELLO_MAC_ARM64: &[u8] = include_bytes!("../../fixtures/hello_mac_arm64");
const HELLO_MAC_X64: &[u8] = include_bytes!("../../fixtures/hello_mac_x64");

#[test]
fn hello_mac_arm64() {
    helpers::setup();
    let path = write_hello_fixture("mwemu_hello_mac_arm64", HELLO_MAC_ARM64);

    helpers::run_with_timeout(helpers::TEST_BUDGET, move || {
        let mut emu = emu_aarch64();
        emu.load_code(path.to_str().unwrap());

        assert!(
            emu.cfg.arch.is_aarch64(),
            "expected Mach-O aarch64 dispatch"
        );
        let pc = emu.regs_aarch64().pc;
        assert!(pc >= 0x100000000, "entry 0x{:x} should be in __TEXT", pc);

        for _ in 0..HELLO_WORLD_STEPS {
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
    let path = write_hello_fixture("mwemu_hello_mac_x64", HELLO_MAC_X64);

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
