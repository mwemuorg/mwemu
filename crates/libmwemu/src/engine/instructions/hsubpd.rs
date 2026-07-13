use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// HSUBPD: horizontal f64 pairwise operation (dest pairs -> low, src -> high).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    let d0 = f64::from_bits((dest & 0xffff_ffff_ffff_ffff) as u64);
    let d1 = f64::from_bits(((dest >> 64) & 0xffff_ffff_ffff_ffff) as u64);
    let s0 = f64::from_bits((src & 0xffff_ffff_ffff_ffff) as u64);
    let s1 = f64::from_bits(((src >> 64) & 0xffff_ffff_ffff_ffff) as u64);
    let out = [d0 - d1, s0 - s1];
    for i in 0..2 {
        result |= (out[i].to_bits() as u128) << (i * 64);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
