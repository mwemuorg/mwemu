use iced_x86::Instruction;

use crate::color;
use crate::emu::{Emu, decoded_instruction::DecodedInstruction};
use crate::engine::instructions::simd::{self, Shift};

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    simd::shift(emu, ins, u32::BITS, Shift::Arithmetic)
}
