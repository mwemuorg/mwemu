use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;
// VCMPSD dest, src1, src2, imm8: low-f64 compare -> all-ones/0; [127:64] from src1.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let s1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let s2 = emu.get_operand_xmm_value_128(ins, 2, true).unwrap_or(0);
    let imm = emu.get_operand_value(ins, 3, true).unwrap_or(0) as u8;
    let low: u128 = if avx::cmp_pred_f64(f64::from_bits(s1 as u64), f64::from_bits(s2 as u64), imm)
    {
        0xffff_ffff_ffff_ffff
    } else {
        0
    };
    emu.set_operand_xmm_value_128(ins, 0, (s1 & !(0xffff_ffff_ffff_ffffu128)) | low);
    true
}
