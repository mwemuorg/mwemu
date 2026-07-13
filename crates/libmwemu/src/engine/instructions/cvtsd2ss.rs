use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// CVTSD2SS: low f64 -> low f32; bits [127:32] of dest are preserved.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let f = f64::from_bits((src & 0xffff_ffff_ffff_ffff) as u64) as f32;
    let result = (dest & !0xffff_ffffu128) | (f.to_bits() as u128);
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
