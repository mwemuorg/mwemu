use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    avx::scalar32(emu, ins, |x, y| if x > y { x } else { y })
}
