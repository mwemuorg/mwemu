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

            let a_bytes = source1.to_le_bytes();
            let b_bytes = source2.to_le_bytes();

            let mut result = [0u8; 16];

            for i in 0..16 {
                if a_bytes[i] == b_bytes[i] {
                    result[i] = 0xFF;
                } else {
                    result[i] = 0;
                }
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

            for i in 0..32 {
                if bytes1[i] == bytes2[i] {
                    result[i] = 0xFF;
                } else {
                    result[i] = 0;
                }
            }

            let result256: regs64::U256 = regs64::U256::from_little_endian(&result);

            emu.set_operand_ymm_value_256(ins, 0, result256);
        }
        _ => unreachable!(""),
    }
    true
}
