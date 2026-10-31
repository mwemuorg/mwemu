//! MMX (`mm` register) forms of integer compare instructions share the
//! mnemonic with the SSE forms; the handlers used to `assert!` a 128-bit
//! operand and abort the process on the 64-bit encoding.

use crate::*;

fn mm_op(opcode: &[u8], mm0: u64, mm1: u64) -> u64 {
    let mut emu = emu64();
    emu.load_code_bytes(opcode);
    emu.regs_mut().mm0 = mm0 as u128;
    emu.regs_mut().mm1 = mm1 as u128;
    assert!(emu.step(), "instruction {:x?} should emulate", opcode);
    emu.regs().mm0 as u64
}

#[test]
fn pcmpgtb_mm_signed_byte_compare() {
    // lanes (low..high): 5>1, -128>1, 127>127, 0>-1, 0>0, 1>0, -1>-2, 0x7f>0x80
    let a = u64::from_le_bytes([0x05, 0x80, 0x7f, 0x00, 0x00, 0x01, 0xff, 0x7f]);
    let b = u64::from_le_bytes([0x01, 0x01, 0x7f, 0xff, 0x00, 0x00, 0xfe, 0x80]);
    let r = mm_op(&[0x0f, 0x64, 0xc1], a, b);
    assert_eq!(
        r,
        u64::from_le_bytes([0xff, 0x00, 0x00, 0xff, 0x00, 0xff, 0xff, 0xff])
    );
}

#[test]
fn pcmpgtw_mm_signed_word_compare() {
    // lanes: 5>1, -1>1, 0x7fff>0x7fff, 0>-1
    let a = u64::from_le_bytes([0x05, 0x00, 0xff, 0xff, 0xff, 0x7f, 0x00, 0x00]);
    let b = u64::from_le_bytes([0x01, 0x00, 0x01, 0x00, 0xff, 0x7f, 0xff, 0xff]);
    let r = mm_op(&[0x0f, 0x65, 0xc1], a, b);
    assert_eq!(
        r,
        u64::from_le_bytes([0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff])
    );
}
