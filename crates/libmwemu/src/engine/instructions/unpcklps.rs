use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// UNPCKLPS: interleave low dwords -> [d0, s0, d1, s1].
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let d = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let s = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let g = |v: u128, n: u32| (v >> (32 * n)) & 0xffffffff;
    let r = g(d, 0) | (g(s, 0) << 32) | (g(d, 1) << 64) | (g(s, 1) << 96);
    emu.set_operand_xmm_value_128(ins, 0, r);
    true
}
