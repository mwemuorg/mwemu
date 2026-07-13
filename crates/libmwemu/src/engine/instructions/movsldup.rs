use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// MOVSLDUP: duplicate even-indexed dwords -> [s0,s0,s2,s2].
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let s = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let g = |n: u32| (s >> (32 * n)) & 0xffffffff;
    emu.set_operand_xmm_value_128(ins, 0, g(0) | (g(0) << 32) | (g(2) << 64) | (g(2) << 96));
    true
}
