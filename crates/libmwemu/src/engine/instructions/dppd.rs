use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// DPPD: dot product of 2 f64 lanes; imm8[5:4] inputs, imm8[1:0] outputs.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let imm = emu.get_operand_value(ins, 2, true).unwrap_or(0) as u32;
    let mut sum = 0f64;
    for i in 0..2 {
        if (imm >> (4 + i)) & 1 == 1 {
            let a = f64::from_bits(((dest >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64);
            let b = f64::from_bits(((src >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64);
            sum += a * b;
        }
    }
    let mut result = 0u128;
    for i in 0..2 {
        if (imm >> i) & 1 == 1 {
            result |= (sum.to_bits() as u128) << (i * 64);
        }
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
