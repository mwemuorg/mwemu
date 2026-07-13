use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VSHUFPS: VEX binary op with imm8 (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop_imm(emu, ins, |a, b, imm| {
        let g = |v: u128, n: u32| (v >> (32 * n)) & 0xffffffff;
        g(a, (imm & 3) as u32)
            | (g(a, ((imm >> 2) & 3) as u32) << 32)
            | (g(b, ((imm >> 4) & 3) as u32) << 64)
            | (g(b, ((imm >> 6) & 3) as u32) << 96)
    })
}
