use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPMAXUB: VEX vertical op (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| avx::lanes(a, b, 8, |x, y| x.max(y)))
}
