use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let mut val: u64 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };
    let mut mask: u64 = match emu.get_operand_value(ins, 2, true) {
        Some(v) => v,
        None => return false,
    };

    let mut result = 0;
    let mut bit = 0;

    while mask != 0 {
        if mask & 1 != 0 {
            if val & 1 != 0 {
                result |= 1 << bit;
            }
            val >>= 1;
        }
        bit += 1;
        mask >>= 1;
    }

    if !emu.set_operand_value(ins, 0, result) {
        return false;
    }
    true
}
