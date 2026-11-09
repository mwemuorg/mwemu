use iced_x86::Instruction;

use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let Some(source) = emu.get_operand_xmm_value_128(ins, 1, true) else {
        return false;
    };
    let mut result = 0;
    for byte in 0..emu.get_operand_sz(ins, 1) / u8::BITS {
        let sign = (source >> (byte * u8::BITS + u8::BITS - 1)) & 1;
        result |= (sign as u64) << byte;
    }
    emu.set_operand_value(ins, 0, result)
}
