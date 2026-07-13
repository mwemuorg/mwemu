use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPMINUW: VEX vertical op (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| avx::lanes(a, b, 16, |x, y| x.min(y)))
}
