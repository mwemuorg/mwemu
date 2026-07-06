use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let source0 = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let source1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;

    for i in 0..8 {
        let mask = 0xFFFFu128;
        let shift = i * 16;

        let word0 = ((source0 >> shift) & mask) as i16;
        let word1 = ((source1 >> shift) & mask) as i16;
        let product = (word0 as i32) * (word1 as i32);
        let high_word = ((product >> 16) & 0xFFFF) as u128;
        result |= high_word << shift;
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
