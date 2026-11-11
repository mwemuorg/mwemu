use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Red"), &DecodedInstruction::X86(*ins));

    let value = try_operand!(emu.get_operand_value(ins, 0, true));
    emu.fpu_mut().mxcsr = value as u32;
    true
}
