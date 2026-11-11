use iced_x86::Instruction;

use crate::color;
use crate::emu::{Emu, decoded_instruction::DecodedInstruction};
use crate::engine::instructions::simd;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    simd::pack(emu, ins, u16::BITS, true)
}
