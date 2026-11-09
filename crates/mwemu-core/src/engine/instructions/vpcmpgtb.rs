use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::{color, regs64};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _sz: usize, _rep: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    match emu.get_operand_sz(ins, 0) {
        128 => {
            let a = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true)).to_le_bytes();
            let b = try_operand!(emu.get_operand_xmm_value_128(ins, 2, true)).to_le_bytes();
            let mut r = [0u8; 16];
            for i in 0..16 {
                r[i] = if (a[i] as i8) > (b[i] as i8) { 0xFF } else { 0 };
            }
            emu.set_operand_xmm_value_128(ins, 0, u128::from_le_bytes(r))
        }
        256 => {
            let s1 = try_operand!(emu.get_operand_ymm_value_256(ins, 1, true));
            let s2 = try_operand!(emu.get_operand_ymm_value_256(ins, 2, true));
            let a = s1.to_little_endian();
            let b = s2.to_little_endian();
            let mut r = [0u8; 32];
            for i in 0..32 {
                r[i] = if (a[i] as i8) > (b[i] as i8) { 0xFF } else { 0 };
            }
            emu.set_operand_ymm_value_256(ins, 0, regs64::U256::from_little_endian(&r))
        }
        _ => {
            log::warn!("vpcmpgtb: unexpected operand size");
            false
        }
    }
}
