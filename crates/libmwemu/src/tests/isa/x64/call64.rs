use crate::tests::helpers;
use crate::*;

#[test]
// this tests the emu.call64() Microsoft ABI
pub fn call64() {
    helpers::setup();

    let mut emu = emu64();
    let opcodes: Vec<u8> = vec![
        0x55, 0x48, 0x89, 0xe5, 0x89, 0x7d, 0xfc, 0x89, 0x75, 0xf8, 0x8b, 0x55, 0xfc, 0x8b, 0x45,
        0xf8, 0x01, 0xd0, 0x5d, 0xc3,
    ];
    emu.set_verbose(3);
    emu.os = crate::arch::OperatingSystem::Linux; // otherwise I would need to set map files.
    emu.load_code_bytes(&opcodes);
    emu.regs_mut().rax = 0;
    let rax = emu.call64(emu.regs().rip, &[]).unwrap();
    assert_eq!(emu.regs().rip, 0x3c0013);
    // TODO: improve this test with something with microsoft ABI and more than 4 params.
    //assert_eq!(rax, 0x134c);
    //assert_eq!(emu.regs().rax, 0x134c);
}
