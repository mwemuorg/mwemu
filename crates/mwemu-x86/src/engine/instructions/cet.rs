use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// Shadow stacks are not modeled; preserve destinations as on a CPU with CET disabled.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    true
}
