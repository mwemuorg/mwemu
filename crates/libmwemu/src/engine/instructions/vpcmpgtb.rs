use crate::emu::Emu;
use crate::{color, regs64};
use iced_x86::Instruction;

// VPCMPGTB dest, src1, src2 : signed byte compare, 0xFF where src1>src2 else 0.
pub fn execute(emu: &mut Emu, ins: &Instruction, _sz: usize, _rep: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    match emu.get_operand_sz(ins, 0) {
        128 => {
            let a = emu
                .get_operand_xmm_value_128(ins, 1, true)
                .unwrap_or(0)
                .to_le_bytes();
            let b = emu
                .get_operand_xmm_value_128(ins, 2, true)
                .unwrap_or(0)
                .to_le_bytes();
            let mut r = [0u8; 16];
            for i in 0..16 {
                r[i] = if (a[i] as i8) > (b[i] as i8) { 0xFF } else { 0 };
            }
            emu.set_operand_xmm_value_128(ins, 0, u128::from_le_bytes(r));
        }
        256 => {
            let s1 = emu
                .get_operand_ymm_value_256(ins, 1, true)
                .unwrap_or_default();
            let s2 = emu
                .get_operand_ymm_value_256(ins, 2, true)
                .unwrap_or_default();
            let mut a = vec![0u8; 32];
            s1.to_little_endian(&mut a);
            let mut b = vec![0u8; 32];
            s2.to_little_endian(&mut b);
            let mut r = [0u8; 32];
            for i in 0..32 {
                r[i] = if (a[i] as i8) > (b[i] as i8) { 0xFF } else { 0 };
            }
            emu.set_operand_ymm_value_256(ins, 0, regs64::U256::from_little_endian(&r));
        }
        _ => unreachable!(""),
    }
    true
}
