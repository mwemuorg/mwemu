use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;
// VPERMILPD: per-128-lane f64 permute (1 bit per lane); imm8 or variable.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    if ins.op_kind(2) == iced_x86::OpKind::Register || ins.op_kind(2) == iced_x86::OpKind::Memory {
        avx::binop(emu, ins, |a, b| {
            let g = |n: u32| (a >> (n * 64)) & 0xffff_ffff_ffff_ffff;
            let s0 = ((b >> 1) & 1) as u32;
            let s1 = ((b >> 65) & 1) as u32;
            g(s0) | (g(s1) << 64)
        })
    } else {
        avx::unop_imm(emu, ins, |a, imm| {
            let g = |n: u32| (a >> (n * 64)) & 0xffff_ffff_ffff_ffff;
            g((imm & 1) as u32) | (g(((imm >> 1) & 1) as u32) << 64)
        })
    }
}
