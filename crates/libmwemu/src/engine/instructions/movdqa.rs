use iced_x86::Instruction;

use crate::color;
use crate::emu::{Emu, decoded_instruction::DecodedInstruction};

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let Some(value) = emu.get_operand_xmm_value_128(ins, 1, true) else {
        return false;
    };
    emu.set_operand_xmm_value_128(ins, 0, value)
}
