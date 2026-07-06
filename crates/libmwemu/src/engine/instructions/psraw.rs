use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let value1 = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let value6 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    let shift_amount = (value6 & 0xFF) as u32;

    for i in 0..8 {
        let mask = 0xFFFFu128;
        let shift = i * 16;

        let word = ((value1 >> shift) & mask) as i16;
        let shifted_word = (word as i32 >> shift_amount) as i16;

        result |= (shifted_word as u128) << shift;
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
