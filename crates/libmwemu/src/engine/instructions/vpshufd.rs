use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPSHUFD: VEX unary op with imm8 (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::unop_imm(emu, ins, |a, imm| {
        let g = |n: u32| (a >> (32 * (((imm >> (2 * n)) & 3) as u32))) & 0xffffffff;
        g(0) | (g(1) << 32) | (g(2) << 64) | (g(3) << 96)
    })
}
