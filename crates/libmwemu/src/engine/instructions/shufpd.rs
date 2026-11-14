use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    // Bit 0 selects a destination qword for the low result;
    // bit 1 selects a source qword for the high result.
    let imm = try_operand!(emu.get_operand_value(ins, 2, true));

    let low = if imm & 1 != 0 {
        dest >> 64
    } else {
        dest & 0xFFFFFFFFFFFFFFFF
    };
    let high = if imm & 2 != 0 {
        src >> 64
    } else {
        src & 0xFFFFFFFFFFFFFFFF
    };
    let result = (low & 0xFFFFFFFFFFFFFFFF) | ((high & 0xFFFFFFFFFFFFFFFF) << 64);

    emu.set_operand_xmm_value_128(ins, 0, result)
}
