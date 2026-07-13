use crate::arch::x86::regs::U256;
use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;
// VPERMQ dest, src, imm8: permute the four 64-bit lanes across the full 256 bits.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let (lo, hi) = avx::to_pair(
        emu.get_operand_ymm_value_256(ins, 1, true)
            .unwrap_or(U256::from(0)),
    );
    let imm = emu.get_operand_value(ins, 2, true).unwrap_or(0) as u32;
    let q = |n: u32| {
        if n < 2 {
            (lo >> (n * 64)) & 0xffff_ffff_ffff_ffff
        } else {
            (hi >> ((n - 2) * 64)) & 0xffff_ffff_ffff_ffff
        }
    };
    let nlo = q(imm & 3) | (q((imm >> 2) & 3) << 64);
    let nhi = q((imm >> 4) & 3) | (q((imm >> 6) & 3) << 64);
    emu.set_operand_ymm_value_256(ins, 0, avx::from_pair(nlo, nhi));
    true
}
