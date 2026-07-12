use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let destination = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let shift_amount = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);

    // PSRLQ shifts each 64-bit lane independently; a count >= 64 zeroes the lane.
    let count = shift_amount as u64;
    let result = if count >= 64 {
        0
    } else {
        let lo = ((destination as u64) >> count) as u128;
        let hi = (((destination >> 64) as u64) >> count) as u128;
        lo | (hi << 64)
    };

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
