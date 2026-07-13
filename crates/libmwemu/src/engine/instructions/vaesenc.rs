use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::aes;
use iced_x86::Instruction;
// VAESENC: AES encrypt round (VEX 3-operand: state, key).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let state = aes::to_bytes(emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0));
    let key = emu.get_operand_xmm_value_128(ins, 2, true).unwrap_or(0);
    let mut s = aes::shift_rows(&state);
    aes::sub_bytes(&mut s);
    let s = aes::mix_columns(&s);
    emu.set_operand_xmm_value_128(ins, 0, aes::from_bytes(s) ^ key);
    true
}
