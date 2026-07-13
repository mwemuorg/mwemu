use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// VCVTSD2SS dest, src1, src2: low f64->f32; [127:32] from src1.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let s1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let s2 = emu.get_operand_xmm_value_128(ins, 2, true).unwrap_or(0);
    let f = f64::from_bits(s2 as u64) as f32;
    emu.set_operand_xmm_value_128(ins, 0, (s1 & !0xffff_ffffu128) | (f.to_bits() as u128));
    true
}
