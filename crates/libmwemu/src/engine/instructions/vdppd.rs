use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VDPPD: VEX op with imm8.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop_imm(emu, ins, |a, b, imm| {
        let mut sum = 0f64;
        for i in 0..2u32 {
            if (imm >> (4 + i)) & 1 == 1 {
                sum += f64::from_bits(((a >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64)
                    * f64::from_bits(((b >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64);
            }
        }
        let mut r = 0u128;
        for i in 0..2u32 {
            if (imm >> i) & 1 == 1 {
                r |= (sum.to_bits() as u128) << (i * 64);
            }
        }
        r
    })
}
