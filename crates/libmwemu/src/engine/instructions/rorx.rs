use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let src = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };
    let imm = match emu.get_operand_value(ins, 2, true) {
        Some(v) => v as u32,
        None => return false,
    };

    let result = match emu.get_operand_sz(ins, 0) {
        32 => (src as u32).rotate_right(imm & 31) as u64,
        _ => src.rotate_right(imm & 63),
    };

    emu.set_operand_value(ins, 0, result)
}
