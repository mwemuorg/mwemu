use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    emu.flags_mut().f_tf = false;
    let flags = emu.flags().dump() as u64;
    if !emu.stack_push64(flags) {
        return false;
    }
    true
}
