use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// MOVUPD: move 128 bits (unaligned) from src to dest.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let s = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    emu.set_operand_xmm_value_128(ins, 0, s);
    true
}
