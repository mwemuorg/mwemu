use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VMOVSHDUP: VEX op.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::unop(emu, ins, |a| {
        let g = |n: u32| (a >> (32 * n)) & 0xffffffff;
        g(1) | (g(1) << 32) | (g(3) << 64) | (g(3) << 96)
    })
}
