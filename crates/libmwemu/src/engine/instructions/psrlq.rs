use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let destination = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let shift_amount = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let result = destination.wrapping_shr(shift_amount as u32);

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
