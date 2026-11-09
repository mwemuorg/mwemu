use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    avx::unop(emu, ins, |a| {
        let g = |n: u32| (a >> (32 * n)) & 0xffffffff;
        g(0) | (g(0) << 32) | (g(2) << 64) | (g(2) << 96)
    })
}
