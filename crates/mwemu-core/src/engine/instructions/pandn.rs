use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let value0 = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let value1 = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let result = (!value0) & value1;

    emu.set_operand_xmm_value_128(ins, 0, result)
}
