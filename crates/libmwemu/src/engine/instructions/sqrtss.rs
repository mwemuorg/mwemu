use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::fp;
use iced_x86::Instruction;

// SQRTSS: scalar f32 square root of the low source lane with the
// x86 NaN rules (see `fp`); the upper lanes of the destination are preserved.
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
    let r = fp::sqrt_f32(f32::from_bits(src as u32));
    let result = (dest & !0xffff_ffff_u128) | (r.to_bits() as u128);
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
