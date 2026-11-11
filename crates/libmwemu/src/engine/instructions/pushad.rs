use iced_x86::Instruction;

use crate::color;
use crate::emu::{Emu, decoded_instruction::DecodedInstruction};

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Blue"), &DecodedInstruction::X86(*ins));
    let sp = emu.sp();
    let values = [
        emu.regs().get_eax(),
        emu.regs().get_ecx(),
        emu.regs().get_edx(),
        emu.regs().get_ebx(),
        emu.regs().get_esp(),
        emu.regs().get_ebp(),
        emu.regs().get_esi(),
        emu.regs().get_edi(),
    ];
    for value in values {
        if !emu.push_instruction(value, u32::BITS) {
            emu.set_sp(sp);
            return false;
        }
    }
    true
}
