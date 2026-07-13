use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPMOVZXBD: widen 8-bit lanes to 32-bit.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::pmovx(emu, ins, 8, 32, false)
}
