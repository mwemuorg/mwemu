use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::{avx, fp};
use iced_x86::Instruction;

// SQRTPD: per-lane f64 square root with the x86 NaN rules (see `fp`).
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
    let _ = dest;
    emu.set_operand_xmm_value_128(ins, 0, avx::fpd(src, src, |x, _| fp::sqrt_f64(x)));
    true
}
