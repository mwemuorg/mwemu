use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// SHA1RNDS4 DEST, SRC, imm8: perform four SHA-1 rounds; imm8[1:0] selects the
// round function and constant.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let d = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let s = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let imm = emu.get_operand_value(ins, 2, true).unwrap_or(0) as u32 & 3;
    let dw = |v: u128, i: u32| ((v >> (i * 32)) & 0xffffffff) as u32;
    let (mut a, mut b, mut c, mut d0) = (dw(d, 3), dw(d, 2), dw(d, 1), dw(d, 0));
    let w = [dw(s, 3), dw(s, 2), dw(s, 1), dw(s, 0)];
    let k: u32 = match imm {
        0 => 0x5A827999,
        1 => 0x6ED9EBA1,
        2 => 0x8F1BBCDC,
        _ => 0xCA62C1D6,
    };
    let f = |b: u32, c: u32, d: u32| -> u32 {
        match imm {
            0 => (b & c) | (!b & d),
            2 => (b & c) | (b & d) | (c & d),
            _ => b ^ c ^ d,
        }
    };
    let mut e = 0u32;
    for &wi in w.iter() {
        let t = a
            .rotate_left(5)
            .wrapping_add(f(b, c, d0))
            .wrapping_add(e)
            .wrapping_add(k)
            .wrapping_add(wi);
        e = d0;
        d0 = c;
        c = b.rotate_left(30);
        b = a;
        a = t;
    }
    emu.set_operand_xmm_value_128(
        ins,
        0,
        (d0 as u128) | ((c as u128) << 32) | ((b as u128) << 64) | ((a as u128) << 96),
    );
    true
}
