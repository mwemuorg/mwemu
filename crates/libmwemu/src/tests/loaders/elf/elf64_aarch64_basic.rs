use crate::tests::helpers;
use crate::*;

/// Full ELF64 AArch64 binary for:
///   mov x0, #1
///   mov x1, #1
///   add x2, x0, x1
///   mov x8, #93
///   svc #0
/// Compiled with: zig cc -target aarch64-linux -static -nostdlib -Wl,--strip-all
const ELF64_AARCH64_ADD: &[u8] = include_bytes!("../../fixtures/elf64_aarch64_add.bin");

#[test]
fn elf64_aarch64_load_and_execute() {
    helpers::setup();

    let tmp = std::env::temp_dir().join("mwemu_test_elf64_aarch64_add.bin");
    std::fs::write(&tmp, ELF64_AARCH64_ADD).unwrap();

    let mut emu = emu_aarch64();
    emu.load_code(tmp.to_str().unwrap());

    assert!(emu.cfg.arch.is_aarch64());

    // Step through: mov x0, #1
    emu.step();
    assert_eq!(emu.regs_aarch64().x[0], 1);

    // mov x1, #1
    emu.step();
    assert_eq!(emu.regs_aarch64().x[1], 1);

    // add x2, x0, x1
    emu.step();
    assert_eq!(emu.regs_aarch64().x[2], 2);

    // mov x8, #93 (exit syscall number)
    emu.step();
    assert_eq!(emu.regs_aarch64().x[8], 93);

    // svc #0 (syscall)
    emu.step();
}
