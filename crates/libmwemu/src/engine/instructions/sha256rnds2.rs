use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// SHA256RNDS2 DEST, SRC, <XMM0>: two SHA-256 rounds; XMM0 supplies the two
// precomputed K+W values.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let d = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let s = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let xmm0 = emu.regs().xmm0;
    let dw = |v: u128, i: u32| ((v >> (i * 32)) & 0xffffffff) as u32;
    let (mut a, mut b, mut c, mut dd) = (dw(s, 3), dw(s, 2), dw(d, 3), dw(d, 2));
    let (mut e, mut f, mut g, mut h) = (dw(s, 1), dw(s, 0), dw(d, 1), dw(d, 0));
    for round in 0..2 {
        let wk = dw(xmm0, round);
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ (!e & g);
        let t1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(wk);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let t2 = s0.wrapping_add(maj);
        h = g;
        g = f;
        f = e;
        e = dd.wrapping_add(t1);
        dd = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }
    emu.set_operand_xmm_value_128(
        ins,
        0,
        (f as u128) | ((e as u128) << 32) | ((b as u128) << 64) | ((a as u128) << 96),
    );
    true
}
