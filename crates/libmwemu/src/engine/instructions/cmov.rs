use iced_x86::Instruction;

use crate::emu::Emu;

pub fn conditional_move(emu: &mut Emu, ins: &Instruction, condition: bool) -> bool {
    // CMOV reads memory even when the condition is false; a fault must leave DEST intact.
    let source = try_operand!(emu.get_operand_value(ins, 1, true));
    let value = if condition {
        source
    } else {
        try_operand!(emu.get_operand_value(ins, 0, true))
    };
    // The ordinary setter also handles zero-extension of a 32-bit destination.
    emu.set_operand_value(ins, 0, value)
}
