use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// VROUNDSS dest, src1, src2, imm8: low round(src2); [127:32] from src1.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let s1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let s2 = emu.get_operand_xmm_value_128(ins, 2, true).unwrap_or(0);
    let imm = emu.get_operand_value(ins, 3, true).unwrap_or(0) as u8;
    let m = if imm & 4 != 0 { 0 } else { imm & 3 };
    let x = f32::from_bits((s2 & 0xffffffff) as _);
    let v = match m {
        0 => x.round_ties_even(),
        1 => x.floor(),
        2 => x.ceil(),
        _ => x.trunc(),
    };
    emu.set_operand_xmm_value_128(ins, 0, (s1 & !(0xffffffff as u128)) | (v.to_bits() as u128));
    true
}
