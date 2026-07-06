use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let value0 = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0); // xmm1
    let value1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0); // xmm5
    let result = (!value0) & value1;

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
