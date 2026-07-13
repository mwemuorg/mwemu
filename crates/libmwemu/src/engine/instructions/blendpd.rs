use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// BLENDPD: per-lane select from src when the imm8 bit is set, else keep dest.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let imm = emu.get_operand_value(ins, 2, true).unwrap_or(0);
    let mut result = 0u128;
    for i in 0..2 {
        let shift = i * 64;
        let lane = if (imm >> i) & 1 == 1 {
            (src >> shift) & 0xffff_ffff_ffff_ffff
        } else {
            (dest >> shift) & 0xffff_ffff_ffff_ffff
        };
        result |= lane << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
