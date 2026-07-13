use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// VCVTSI2SS dest, src1, r/m int: low = int->f32; [127:32] from src1.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let s1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let iv = emu.get_operand_value(ins, 2, true).unwrap_or(0);
    let f = if emu.get_operand_sz(ins, 2) == 64 {
        iv as i64 as f32
    } else {
        iv as u32 as i32 as f32
    };
    emu.set_operand_xmm_value_128(ins, 0, (s1 & !0xffff_ffffu128) | (f.to_bits() as u128));
    true
}
