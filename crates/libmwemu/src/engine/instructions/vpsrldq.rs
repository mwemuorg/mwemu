use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::{avx, simd};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    avx::unop_imm(emu, ins, |value, count| {
        simd::shift_bytes_right(value, count as u64)
    })
}
