use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// MPSADBW: 8 word results, each the sum of |dest - src| over a 4-byte window;
// imm8[2] picks the src block, imm8[1:0] picks the dest block.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let imm = emu.get_operand_value(ins, 2, true).unwrap_or(0) as u32;
    let src_off = ((imm >> 2) & 1) * 4;
    let dst_off = (imm & 3) * 4;
    let byte = |v: u128, i: u32| -> i32 {
        if i < 16 {
            ((v >> (i * 8)) & 0xff) as i32
        } else {
            0
        }
    };
    let mut result = 0u128;
    for j in 0..8u32 {
        let mut sum = 0u32;
        for k in 0..4u32 {
            sum += (byte(dest, dst_off + j + k) - byte(src, src_off + k)).unsigned_abs();
        }
        result |= (sum as u128) << (j * 16);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
