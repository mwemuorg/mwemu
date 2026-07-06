use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    // PSLLDQ xmm, imm8 — byte shift left. The count is an *immediate*, not an
    // xmm operand, and a count >= 16 zeroes the register (no modulo wrap).
    let value0 = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let shift_bytes = emu.get_operand_value(ins, 1, true).unwrap_or(0);

    let result = if shift_bytes >= 16 {
        0u128
    } else {
        value0 << (shift_bytes * 8)
    };

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
