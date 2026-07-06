use crate::emu::Emu;
use crate::{color, regs64};
use iced_x86::Instruction;

// VPADDB dest, src1, src2 : packed byte-wise wrapping addition.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
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
                r[i] = a[i].wrapping_add(b[i]);
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
                r[i] = a[i].wrapping_add(b[i]);
            }
            emu.set_operand_ymm_value_256(ins, 0, regs64::U256::from_little_endian(&r));
        }
        _ => unreachable!(""),
    }
    true
}
