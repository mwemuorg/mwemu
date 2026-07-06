use crate::tests::helpers;
use crate::*;

#[test]
// tests syscalls64
pub fn elf64lin_syscall64() {
    helpers::setup();

    let mut emu = emu64();
    emu.load_code(&helpers::test_data_path("elf64lin_syscall64.bin"));
    emu.run_to(80000);
    assert_eq!(emu.regs().r12, 549);
}
