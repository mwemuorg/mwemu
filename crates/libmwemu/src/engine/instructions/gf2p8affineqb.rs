use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// GF2P8AFFINEQB: per-byte GF(2) affine transform A*x^{-1?}+b; the 8x8 matrix A is the
// src qword (byte[7-i] is row i), b is imm8.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let imm = emu.get_operand_value(ins, 2, true).unwrap_or(0) as u8;
    let mut result = 0u128;
    for j in 0..16u32 {
        let x = ((dest >> (j * 8)) & 0xff) as u8;
        let block = j / 8;
        let mut out = 0u8;
        for i in 0..8u32 {
            let row = ((src >> ((block * 8 + (7 - i)) * 8)) & 0xff) as u8;
            let bit = ((row & x).count_ones() & 1) as u8 ^ ((imm >> i) & 1);
            out |= bit << i;
        }
        result |= (out as u128) << (j * 8);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
