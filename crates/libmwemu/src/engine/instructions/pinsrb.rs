use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let s = (try_operand!(emu.get_operand_value(ins, 1, true)) as u128) & 0xff;
    let idx = (try_operand!(emu.get_operand_value(ins, 2, true)) as u32) & 0xf;
    let shift = idx * 8;
    let result = (dest & !(0xff_u128 << shift)) | (s << shift);
    emu.set_operand_xmm_value_128(ins, 0, result)
}
