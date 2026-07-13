use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VUNPCKLPS: VEX vertical op (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        let g = |v: u128, n: u32| (v >> (32 * n)) & 0xffffffff;
        g(a, 0) | (g(b, 0) << 32) | (g(a, 1) << 64) | (g(b, 1) << 96)
    })
}
