use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    avx::scalar32(emu, ins, |_, y| 1.0f32 / y.sqrt())
}
