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
        let mask = 0xFFFFu128;
        let shift = i * 16;

        let word0 = ((value0 >> (shift + 48)) & mask) as u16;
        let word1 = ((value1 >> (shift + 48)) & mask) as u16;

        result |= (word0 as u128) << (i * 32);
        result |= (word1 as u128) << (i * 32 + 16);
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
