use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// VPINSRQ dest, src1, r/m, imm8: src1 with the selected 64-bit lane replaced.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let s1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let v = (emu.get_operand_value(ins, 2, true).unwrap_or(0) as u128) & 0xffff_ffff_ffff_ffff;
    let idx = (emu.get_operand_value(ins, 3, true).unwrap_or(0) as u32) & 1;
    let sh = idx * 64;
    emu.set_operand_xmm_value_128(
        ins,
        0,
        (s1 & !((0xffff_ffff_ffff_ffff as u128) << sh)) | (v << sh),
    );
    true
}
