use crate::tests::helpers;
use crate::*;

#[test]
// this tests the emu.call32()
pub fn call32() {
    helpers::setup();

    let mut emu = emu32();
    let opcodes: Vec<u8> = vec![
        //TODO: test it with 7 parameters
        0x55, 0x89, 0xe5, 0x83, 0xec, 0x50, 0xb8, 0x37, 0x13, 0x00, 0x00, 0x83, 0xf0, 0x7b, 0xc9,
        0xc3,
    ];
    emu.set_verbose(3);
    emu.os = crate::arch::OperatingSystem::Linux; // otherwise I would need to set map files.
    emu.load_code_bytes(&opcodes);
    emu.regs_mut().rax = 0;
    let eax = emu.call32(emu.regs().rip, &[]).unwrap();
    assert_eq!(emu.regs().get_eax() as u32, eax);
    assert_eq!(eax, 0x134c);
    assert_eq!(emu.regs().rax, 0x134c);
}
