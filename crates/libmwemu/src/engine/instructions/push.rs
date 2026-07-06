use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    let value = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    emu.show_instruction_pushpop(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
        value,
    );

    if emu.cfg.is_x64() {
        if !emu.stack_push64(value) {
            return false;
        }
    } else if !emu.stack_push32(to32!(value)) {
        return false;
    }
    true
}
