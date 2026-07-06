use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    // Obtener los valores de los registros XMM (128 bits cada uno)
    let value0 = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0); // xmm6
    let value1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0); // xmm5
    let mut result = 0u128;

    for i in 0..8 {
        let mask = 0xFFFFu128;
        let shift = i * 16;
        let word0 = ((value0 >> shift) & mask) as i16;
        let word1 = ((value1 >> shift) & mask) as i16;
        let diff = word0.saturating_sub(word1);

        result |= (diff as u128 & mask) << shift;
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
