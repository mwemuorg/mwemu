use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// HSUBPS: horizontal f32 pairwise operation (dest pairs -> low, src -> high).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    let d0 = f32::from_bits(((dest >> 0) & 0xffffffff) as u32);
    let d1 = f32::from_bits(((dest >> 32) & 0xffffffff) as u32);
    let d2 = f32::from_bits(((dest >> 64) & 0xffffffff) as u32);
    let d3 = f32::from_bits(((dest >> 96) & 0xffffffff) as u32);
    let s0 = f32::from_bits(((src >> 0) & 0xffffffff) as u32);
    let s1 = f32::from_bits(((src >> 32) & 0xffffffff) as u32);
    let s2 = f32::from_bits(((src >> 64) & 0xffffffff) as u32);
    let s3 = f32::from_bits(((src >> 96) & 0xffffffff) as u32);
    let out = [d0 - d1, d2 - d3, s0 - s1, s2 - s3];
    for i in 0..4 {
        result |= (out[i].to_bits() as u128) << (i * 32);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
