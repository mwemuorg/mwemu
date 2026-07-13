use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// SHA256MSG1: DEST.dword[i] += sigma0(dword[i+1]); dword[4] comes from SRC.dword[0].
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let d = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let s = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let dw = |v: u128, i: u32| ((v >> (i * 32)) & 0xffffffff) as u32;
    let s0 = |x: u32| x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3);
    let r0 = dw(d, 0).wrapping_add(s0(dw(d, 1)));
    let r1 = dw(d, 1).wrapping_add(s0(dw(d, 2)));
    let r2 = dw(d, 2).wrapping_add(s0(dw(d, 3)));
    let r3 = dw(d, 3).wrapping_add(s0(dw(s, 0)));
    emu.set_operand_xmm_value_128(
        ins,
        0,
        (r0 as u128) | ((r1 as u128) << 32) | ((r2 as u128) << 64) | ((r3 as u128) << 96),
    );
    true
}
