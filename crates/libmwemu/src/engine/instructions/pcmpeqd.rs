use iced_x86::Instruction;

use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::simd;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    simd::binary_lanes(
        emu,
        ins,
        u32::BITS,
        |a, b| {
            if a == b { u128::MAX } else { 0 }
        },
    )
}
