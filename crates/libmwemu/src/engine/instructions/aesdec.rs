use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::aes;
use iced_x86::Instruction;
// AESDEC: one inverse AES round: InvShiftRows, InvSubBytes, InvMixColumns, XOR key.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let state = aes::to_bytes(emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0));
    let key = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut s = aes::inv_shift_rows(&state);
    aes::inv_sub_bytes(&mut s);
    let s = aes::inv_mix_columns(&s);
    emu.set_operand_xmm_value_128(ins, 0, aes::from_bytes(s) ^ key);
    true
}
