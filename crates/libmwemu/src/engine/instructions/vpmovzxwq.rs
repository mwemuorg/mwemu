use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPMOVZXWQ: widen 16-bit lanes to 64-bit.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::pmovx(emu, ins, 16, 64, false)
}
