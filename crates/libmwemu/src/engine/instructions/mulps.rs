use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::{avx, fp};
use iced_x86::Instruction;

// MULPS: per-lane f32 `*` with the x86 NaN rules (see `fp`).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = match emu.get_operand_xmm_value_128(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };
    let src = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };
    emu.set_operand_xmm_value_128(ins, 0, avx::fps(dest, src, fp::mul_f32));
    true
}
