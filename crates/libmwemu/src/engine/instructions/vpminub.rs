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
            let source1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory xmm 1 source operand");
                    return false;
                }
            };

            let source2 = match emu.get_operand_xmm_value_128(ins, 2, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory xmm 2 source operand");
                    return false;
                }
            };

            let mut result: u128 = 0;
            for i in 0..16 {
                let byte1 = (source1 >> (8 * i)) & 0xFF;
                let byte2 = (source2 >> (8 * i)) & 0xFF;
                let min_byte = byte1.min(byte2);
                result |= min_byte << (8 * i);
            }

            emu.set_operand_xmm_value_128(ins, 0, result);
        }
        256 => {
            let source1 = match emu.get_operand_ymm_value_256(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory ymm 1 source operand");
                    return false;
                }
            };

            let source2 = match emu.get_operand_ymm_value_256(ins, 2, true) {
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

            for i in 0..32 {
                result[i] = bytes1[i].min(bytes2[i]);
            }

            let result256: regs64::U256 = regs64::U256::from_little_endian(&result);

            emu.set_operand_ymm_value_256(ins, 0, result256);
        }
        _ => unreachable!(""),
    }
    true
}
