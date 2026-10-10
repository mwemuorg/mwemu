use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// MULSD: scalar f64 multiplication on the low lane; the upper bits of the
// destination are preserved.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let a = f64::from_bits((dest & 0xffff_ffff_ffff_ffff_u128) as _);
    let b = f64::from_bits((src & 0xffff_ffff_ffff_ffff_u128) as _);
    let r: f64 = a * b;
    let result =
        (dest & !0xffff_ffff_ffff_ffff_u128) | (r.to_bits() as u128 & 0xffff_ffff_ffff_ffff_u128);
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
