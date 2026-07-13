use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::aes;
use iced_x86::Instruction;
// AESKEYGENASSIST dest, src, imm8: key-schedule assist using SubWord/RotWord.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let rcon = (emu.get_operand_value(ins, 2, true).unwrap_or(0) as u32) & 0xff;
    let x1 = (src >> 32) as u32;
    let x3 = (src >> 96) as u32;
    let s1 = aes::sub_word(x1);
    let s3 = aes::sub_word(x3);
    let r0 = s1 as u128;
    let r1 = (s1.rotate_right(8) ^ rcon) as u128;
    let r2 = s3 as u128;
    let r3 = (s3.rotate_right(8) ^ rcon) as u128;
    emu.set_operand_xmm_value_128(ins, 0, r0 | (r1 << 32) | (r2 << 64) | (r3 << 96));
    true
}
