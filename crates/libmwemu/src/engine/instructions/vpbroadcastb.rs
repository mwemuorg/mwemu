use crate::emu::Emu;
use crate::{color, regs64};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let byte: u8 = match emu.get_operand_sz(ins, 1) {
        128 => {
            let source = match emu.get_operand_xmm_value_128(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory xmm 1 source operand");
                    return false;
                }
            };

            (source & 0xff) as u8
        }

        256 => {
            let source = match emu.get_operand_ymm_value_256(ins, 1, true) {
                Some(v) => v,
                None => {
                    log::trace!("error reading memory ymm 1 source operand");
                    return false;
                }
            };

            (source & regs64::U256::from(0xFF)).low_u64() as u8
        }
        _ => unreachable!(""),
    };

    match emu.get_operand_sz(ins, 0) {
        128 => {
            let mut result: u128 = 0;
            for _ in 0..16 {
                result <<= 8;
                result |= byte as u128;
            }
            emu.set_operand_xmm_value_128(ins, 0, result);
        }
        256 => {
            let mut result = regs64::U256::zero();
            for _ in 0..32 {
                result <<= 8;
                result |= regs64::U256::from(byte);
            }
            emu.set_operand_ymm_value_256(ins, 0, result);
        }
        _ => unreachable!(""),
    }
    true
}
