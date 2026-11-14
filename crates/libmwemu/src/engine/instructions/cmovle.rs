use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::cmov::conditional_move;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Orange"), &DecodedInstruction::X86(*ins));
    let condition = emu.flag_zf() || emu.flag_sf() != emu.flag_of();
    conditional_move(emu, ins, condition)
}
