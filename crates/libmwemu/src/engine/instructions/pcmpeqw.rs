use crate::emu::Emu;
use crate::{color, regs64};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    match emu.get_operand_sz(ins, 0) {
        128 => {
            let source1 = match emu.get_operand_xmm_value_128(ins, 0, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory xmm 1 source operand");
                    return false;
                }
            };

            let source2 = match emu.get_operand_xmm_value_128(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory xmm 2 source operand");
                    return false;
                }
            };

            let a_words = source1.to_le_bytes();
            let b_words = source2.to_le_bytes();

            let mut result = [0u8; 16];

            for i in 0..8 {
                let word_a = u16::from_le_bytes([a_words[2 * i], a_words[2 * i + 1]]);
                let word_b = u16::from_le_bytes([b_words[2 * i], b_words[2 * i + 1]]);
                let cmp_result: u16 = if word_a == word_b { 0xFFFF } else { 0x0000 };
                let [low, high] = cmp_result.to_le_bytes();
                result[2 * i] = low;
                result[2 * i + 1] = high;
            }
            let result = u128::from_le_bytes(result);
            emu.set_operand_xmm_value_128(ins, 0, result);
        }
        256 => {
            let source1 = match emu.get_operand_ymm_value_256(ins, 0, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory ymm 1 source operand");
                    return false;
                }
            };

            let source2 = match emu.get_operand_ymm_value_256(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory ymm 2 source operand");
                    return false;
                }
            };

            let mut bytes1: Vec<u8> = vec![0; 32];
            source1.to_little_endian(&mut bytes1);
            let mut bytes2: Vec<u8> = vec![0; 32];
            source2.to_little_endian(&mut bytes2);
            let mut result = [0u8; 32];

            for i in 0..16 {
                let word1 = u16::from_le_bytes([bytes1[2 * i], bytes1[2 * i + 1]]);
                let word2 = u16::from_le_bytes([bytes2[2 * i], bytes2[2 * i + 1]]);
                let cmp_result = if word1 == word2 { 0xFFFFu16 } else { 0x0000u16 };
                let [low, high] = cmp_result.to_le_bytes();

                result[2 * i] = low;
                result[2 * i + 1] = high;
            }

            let result256: regs64::U256 = regs64::U256::from_little_endian(&result);
            emu.set_operand_ymm_value_256(ins, 0, result256);
        }
        _ => unreachable!(""),
    }
    true
}
