use crate::color;
use crate::emu::{Emu, decoded_instruction::DecodedInstruction};
use crate::exception::types::ExceptionType;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Red"), &DecodedInstruction::X86(*ins));
    if emu.os.is_windows() && !emu.kernel_driver_active() {
        emu.exception(ExceptionType::IllegalInstruction);
    }
    false
}
