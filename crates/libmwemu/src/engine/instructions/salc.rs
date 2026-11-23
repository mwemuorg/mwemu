use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Red"), &DecodedInstruction::X86(*ins));

    let mask = if emu.flag_cf() { u8::MAX } else { 0 };
    emu.regs_mut().set_al(mask.into());
    true
}
