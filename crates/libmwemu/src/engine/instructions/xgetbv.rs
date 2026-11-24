use iced_x86::Instruction;

use crate::arch::x86::XCR0_ENABLED_MASK;
use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::exception::types::ExceptionType;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    match emu.regs().get_ecx() {
        0 => {
            emu.regs_mut().set_edx(0);
            emu.regs_mut().set_eax(XCR0_ENABLED_MASK);
        }
        _ => {
            emu.exception(ExceptionType::BadAddressDereferencing);
            return false;
        }
    }
    true
}
