use crate::emu::Emu;
use crate::{color, exception::types};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let flags: u16 = match emu.maps.read_word(emu.regs().rsp) {
        Some(v) => v,
        None => {
            log::error!("popf cannot read the stack");
            emu.exception(types::ExceptionType::PopfCannotReadStack);
            return false;
        }
    };

    let flags2: u32 = (emu.flags().dump() & 0xffff0000) + (flags as u32);
    emu.flags_mut().load(flags2);
    emu.regs_mut().rsp += 2;
    true
}
