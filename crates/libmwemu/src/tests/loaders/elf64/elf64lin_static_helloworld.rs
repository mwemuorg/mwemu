use crate::tests::helpers;
use crate::*;

#[test]
// this tests a linux 64bits static ELF binary.
pub fn elf64lin_static_helloworld() {
    helpers::setup();

    let mut emu = emu64();

    let sample = helpers::test_data_path("elf64lin_static_helloworld.bin");
    emu.load_code(&sample);
    emu.run(Some(0x44ab87));

    assert_eq!(emu.regs().rcx, 0x4cc2d0);
    assert_eq!(emu.pos, 11236);
}
