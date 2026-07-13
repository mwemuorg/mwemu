use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// DPPS: dot product of f32 lanes; imm8[7:4] selects inputs, imm8[3:0] outputs.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let imm = emu.get_operand_value(ins, 2, true).unwrap_or(0) as u32;
    let mut sum = 0f32;
    for i in 0..4 {
        if (imm >> (4 + i)) & 1 == 1 {
            let a = f32::from_bits(((dest >> (i * 32)) & 0xffffffff) as u32);
            let b = f32::from_bits(((src >> (i * 32)) & 0xffffffff) as u32);
            sum += a * b;
        }
    }
    let mut result = 0u128;
    for i in 0..4 {
        if (imm >> i) & 1 == 1 {
            result |= (sum.to_bits() as u128) << (i * 32);
        }
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
