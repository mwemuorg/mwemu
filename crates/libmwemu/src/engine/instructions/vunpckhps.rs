use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    avx::binop(emu, ins, |a, b| {
        let g = |v: u128, n: u32| (v >> (32 * n)) & 0xffffffff;
        g(a, 2) | (g(b, 2) << 32) | (g(a, 3) << 64) | (g(b, 3) << 96)
    })
}
