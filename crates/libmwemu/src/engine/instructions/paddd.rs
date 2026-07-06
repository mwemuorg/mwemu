use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let value0 = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let value1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);

    let mut result = 0u128;

    for i in 0..4 {
        let mask = 0xFFFFFFFFu128;
        let shift = i * 32;
        let word0 = ((value0 >> shift) & mask) as u32;
        let word1 = ((value1 >> shift) & mask) as u32;
        let sum = word0.wrapping_add(word1);

        result |= (sum as u128) << shift;
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
