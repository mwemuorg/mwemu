use crate::emu::Emu;
use crate::{color, exception::types};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let val: u16 = (emu.flags().dump() & 0xffff) as u16;

    emu.regs_mut().rsp -= 2;

    if !emu.maps.write_word(emu.regs().rsp, val) {
        log::trace!("/!\\ exception writing word at rsp 0x{:x}", emu.regs().rsp);
        emu.exception(types::ExceptionType::WritingWord);
        return false;
    }
    true
}
