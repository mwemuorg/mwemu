use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::aes;
use iced_x86::Instruction;
// AESIMC: dest = InvMixColumns(src).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let src = aes::to_bytes(emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0));
    let s = aes::inv_mix_columns(&src);
    emu.set_operand_xmm_value_128(ins, 0, aes::from_bytes(s));
    true
}
