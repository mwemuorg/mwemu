use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    avx::binop(emu, ins, |a, b| {
        avx::lanes(a, b, 64, |x, y| {
            if (x as u64 as i64) > (y as u64 as i64) {
                u64::MAX as u128
            } else {
                0
            }
        })
    })
}
