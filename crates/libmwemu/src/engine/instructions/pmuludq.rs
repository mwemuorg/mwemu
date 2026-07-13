use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PMULUDQ: multiply the low 32-bit element of each 64-bit lane (u32) into a
// 64-bit product.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for i in 0..2 {
        let a = ((dest >> (i * 64)) & 0xffffffff) as u32 as u32 as u64;
        let b = ((src >> (i * 64)) & 0xffffffff) as u32 as u32 as u64;
        result |= ((a.wrapping_mul(b) as u64) as u128) << (i * 64);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
