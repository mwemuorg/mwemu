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

    for i in 0..8 {
        let mask = 0xFFFFu128;
        let shift = i * 16;
        let word0 = ((value0 >> shift) & mask) as i16;
        let word1 = ((value1 >> shift) & mask) as i16;
        let byte0 = if word0 > 255 {
            255
        } else if word0 < 0 {
            0
        } else {
            word0 as u8
        };
        let byte1 = if word1 > 255 {
            255
        } else if word1 < 0 {
            0
        } else {
            word1 as u8
        };

        result |= (byte0 as u128) << (i * 8);
        result |= (byte1 as u128) << ((i + 8) * 8);
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
