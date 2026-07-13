use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPSHUFHW: VEX unary op with imm8 (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::unop_imm(emu, ins, |a, imm| {
        let lo = a & 0xffff_ffff_ffff_ffff;
        let g = |n: u32| (a >> (64 + 16 * (((imm >> (2 * n)) & 3) as u32))) & 0xffff;
        lo | (g(0) << 64) | (g(1) << 80) | (g(2) << 96) | (g(3) << 112)
    })
}
