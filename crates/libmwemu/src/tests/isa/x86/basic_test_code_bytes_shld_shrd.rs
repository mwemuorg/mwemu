use crate::tests::helpers;
use crate::*;

#[test]
// test shrd shld and load_code_bytes() instead of load_code()
pub fn basic_test_code_bytes_shld_shrd() {
    helpers::setup();

    let shellcode32: [u8; 19] = [
        0xb8, 0x78, 0x56, 0x34, 0x12, // mov eax, 0x12345678
        0xba, 0xf0, 0xde, 0xbc, 0x9a, // mov edx, 0x9abcdef0
        0x0f, 0xa4, 0xc2, 0x04, // shld edx, eax, 4
        0x0f, 0xac, 0xc2, 0x04, // shrd edx, eax, 4
        0xc3, // ret
    ];

    let mut emu = emu32();

    emu.load_code_bytes(&shellcode32);
    emu.run_to(2);
    assert_eq!(emu.regs().get_edx(), 0x9abcdef0);
    emu.step(); // shld edx, eax, 4
    assert_eq!(emu.regs().get_edx(), 0xabcdef01);
    emu.step(); // shrd edx, eax, 4
    assert_eq!(emu.regs().get_edx(), 0x8abcdef0);

    let shellcode64: [u8; 31] = [
        0x48, 0xb8, 0xf0, 0xde, 0xbc, 0x9a, 0x78, 0x56, 0x34,
        0x12, // mov rax, 0x123456789abcdef0
        0x48, 0xba, 0x10, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc,
        0xfe, // mov rdx, 0xfedcba9876543210
        0x48, 0x0f, 0xa4, 0xc2, 0x04, // shld rdx, rax, 4
        0x48, 0x0f, 0xac, 0xc2, 0x04, // shrd rdx, rax, 4
        0xc3, // ret
    ];

    let mut emu = emu64();
    emu.load_code_bytes(&shellcode64);
    emu.run_to(2);
    assert_eq!(emu.regs().rax, 0x123456789abcdef0);
    assert_eq!(emu.regs().rdx, 0xfedcba9876543210);

    emu.step(); // shld rdx, rax, 4
    assert_eq!(emu.regs().rdx, 0xedcba98765432101);

    emu.step(); // shrd rdx, rax, 4
    assert_eq!(emu.regs().rdx, 0x0edcba9876543210);
}
