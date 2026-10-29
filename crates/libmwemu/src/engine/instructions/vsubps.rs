use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use crate::engine::instructions::fp;
use iced_x86::Instruction;

// VSUBPS: VEX vertical op (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| avx::fps(a, b, fp::sub_f32))
}
