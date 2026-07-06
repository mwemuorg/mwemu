use crate::emu::Emu;
use crate::{color, exception::types};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    log::trace!("/!\\ int 3 sigtrap!!!!");
    emu.exception(types::ExceptionType::Int3);
    // If no exception handler is installed, treat int3 as clean process exit
    // rather than continuing. On real Windows without a debugger, int3 terminates.
    if emu.seh() == 0 && emu.veh() == 0 && emu.uef() == 0 {
        emu.stop();
    }
    true
}
