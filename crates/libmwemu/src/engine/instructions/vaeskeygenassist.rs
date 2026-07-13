use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::aes;
use iced_x86::Instruction;
// VAESKEYGENASSIST dest, src, imm8.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let rcon = (emu.get_operand_value(ins, 2, true).unwrap_or(0) as u32) & 0xff;
    let s1 = aes::sub_word((src >> 32) as u32);
    let s3 = aes::sub_word((src >> 96) as u32);
    let r = (s1 as u128)
        | ((s1.rotate_right(8) ^ rcon) as u128) << 32
        | ((s3 as u128) << 64)
        | ((s3.rotate_right(8) ^ rcon) as u128) << 96;
    emu.set_operand_xmm_value_128(ins, 0, r);
    true
}
