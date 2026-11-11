use iced_x86::Instruction;

use crate::color;
use crate::emu::{Emu, decoded_instruction::DecodedInstruction};

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Blue"), &DecodedInstruction::X86(*ins));
    let sp = emu.sp();
    let values = [
        emu.regs().get_ax(),
        emu.regs().get_cx(),
        emu.regs().get_dx(),
        emu.regs().get_bx(),
        emu.regs().get_sp(),
        emu.regs().get_bp(),
        emu.regs().get_si(),
        emu.regs().get_di(),
    ];
    for value in values {
        if !emu.push_instruction(value, u16::BITS) {
            emu.set_sp(sp);
            return false;
        }
    }
    true
}
