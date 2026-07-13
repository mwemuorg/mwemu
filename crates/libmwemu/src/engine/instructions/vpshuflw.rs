use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPSHUFLW: VEX unary op with imm8 (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::unop_imm(emu, ins, |a, imm| {
        let hi = a & !0xffff_ffff_ffff_ffffu128;
        let g = |n: u32| (a >> (16 * (((imm >> (2 * n)) & 3) as u32))) & 0xffff;
        hi | g(0) | (g(1) << 16) | (g(2) << 32) | (g(3) << 48)
    })
}
