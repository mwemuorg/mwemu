use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::aes;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Blue"), &DecodedInstruction::X86(*ins));
    let state = aes::to_bytes(try_operand!(emu.get_operand_xmm_value_128(ins, 0, true)));
    let key = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let mut s = aes::shift_rows(&state);
    aes::sub_bytes(&mut s);
    emu.set_operand_xmm_value_128(ins, 0, aes::from_bytes(s) ^ key)
}
