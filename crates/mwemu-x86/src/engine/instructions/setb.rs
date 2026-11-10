use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Orange"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    if emu.flag_cf() {
        if !emu.set_operand_value(ins, 0, 1) {
            return false;
        }
    } else if !emu.set_operand_value(ins, 0, 0) {
        return false;
    }
    true
}
