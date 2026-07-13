use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;
// VPERMILPS: per-128-lane f32 permute; imm8 selects, or a variable index vector.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    if ins.op_kind(2) == iced_x86::OpKind::Register || ins.op_kind(2) == iced_x86::OpKind::Memory {
        avx::binop(emu, ins, |a, b| {
            let g = |n: u32| (a >> (n * 32)) & 0xffffffff;
            let s = |n: u32| ((b >> (n * 32)) & 3) as u32;
            g(s(0)) | (g(s(1)) << 32) | (g(s(2)) << 64) | (g(s(3)) << 96)
        })
    } else {
        avx::unop_imm(emu, ins, |a, imm| {
            let g = |n: u32| (a >> (n * 32)) & 0xffffffff;
            g((imm & 3) as u32)
                | (g(((imm >> 2) & 3) as u32) << 32)
                | (g(((imm >> 4) & 3) as u32) << 64)
                | (g(((imm >> 6) & 3) as u32) << 96)
        })
    }
}
