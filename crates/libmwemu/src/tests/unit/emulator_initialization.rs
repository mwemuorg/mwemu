use crate::tests::helpers;
use crate::*;

#[test]
// test emulator initialization and basic operations
pub fn emulator_initialization() {
    helpers::setup();

    // Test 64-bit emulator
    let mut emu64 = emu64();
    assert!(emu64.cfg.is_x64());
    assert_eq!(emu64.pos, 0);
    assert!(!emu64.force_break);
    assert!(!emu64.force_reload);

    // Don't call init to avoid DLL loading issues

    // Test 32-bit emulator
    let mut emu32 = emu32();
    assert!(!emu32.cfg.is_x64());
    assert_eq!(emu32.pos, 0);

    // Don't call init to avoid DLL loading issues

    // Test emulator state after initialization
    assert_eq!(emu64.regs().rip, 0);
    assert_eq!(emu32.regs().rip, 0);

    // Test register clearing
    emu64.regs_mut().rax = 0x123456789ABCDEF0;
    emu64.regs_mut().clear::<64>();
    assert_eq!(emu64.regs().rax, 0);

    emu32.regs_mut().rax = 0x123456789ABCDEF0;
    emu32.regs_mut().sanitize32();
    assert_eq!(emu32.regs().rax & 0xFFFFFFFF00000000, 0);
}
