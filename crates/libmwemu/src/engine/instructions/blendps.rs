use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// BLENDPS: per-lane select from src when the imm8 bit is set, else keep dest.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let imm = emu.get_operand_value(ins, 2, true).unwrap_or(0);
    let mut result = 0u128;
    for i in 0..4 {
        let shift = i * 32;
        let lane = if (imm >> i) & 1 == 1 {
            (src >> shift) & 0xffffffff
        } else {
            (dest >> shift) & 0xffffffff
        };
        result |= lane << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
