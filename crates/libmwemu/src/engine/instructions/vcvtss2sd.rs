use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// VCVTSS2SD dest, src1, src2: low f32->f64; [127:64] from src1.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let s1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let s2 = emu.get_operand_xmm_value_128(ins, 2, true).unwrap_or(0);
    let f = f32::from_bits(s2 as u32) as f64;
    emu.set_operand_xmm_value_128(
        ins,
        0,
        (s1 & !(0xffff_ffff_ffff_ffff as u128)) | (f.to_bits() as u128),
    );
    true
}
