use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::simd;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let value = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let count = try_operand!(emu.get_operand_value(ins, 1, true));
    emu.set_operand_xmm_value_128(ins, 0, simd::shift_bytes_left(value, count))
}
