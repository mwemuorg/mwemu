use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// MAXSD: per-lane f64 operation.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = dest & !(0xffff_ffff_ffff_ffff as u128);
    for i in 0..1 {
        let shift = i * 64;
        let a = f64::from_bits(((dest >> shift) & 0xffff_ffff_ffff_ffff) as _);
        let b = f64::from_bits(((src >> shift) & 0xffff_ffff_ffff_ffff) as _);
        let r: f64 = if a > b { a } else { b };
        result |= ((r.to_bits() as u128) & (0xffff_ffff_ffff_ffff as u128)) << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
