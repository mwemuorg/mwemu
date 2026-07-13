use crate::arch::x86::regs::U256;
use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;
// VPERM2I128 dest, src1, src2, imm8: select a 128-bit lane per half (bit3/7 zeroes).
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let (a0, a1) = avx::to_pair(
        emu.get_operand_ymm_value_256(ins, 1, true)
            .unwrap_or(U256::from(0)),
    );
    let (b0, b1) = avx::to_pair(
        emu.get_operand_ymm_value_256(ins, 2, true)
            .unwrap_or(U256::from(0)),
    );
    let imm = emu.get_operand_value(ins, 3, true).unwrap_or(0) as u32;
    let sel = |s: u32| -> u128 {
        match s & 3 {
            0 => a0,
            1 => a1,
            2 => b0,
            _ => b1,
        }
    };
    let lo = if imm & 8 != 0 { 0 } else { sel(imm & 3) };
    let hi = if imm & 0x80 != 0 {
        0
    } else {
        sel((imm >> 4) & 3)
    };
    emu.set_operand_ymm_value_256(ins, 0, avx::from_pair(lo, hi));
    true
}
