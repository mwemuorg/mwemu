use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let x = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let idx = (try_operand!(emu.get_operand_value(ins, 2, true)) as u32) & 0xf;
    let v = ((x >> (idx * 8)) & 0xff) as u64;
    emu.set_operand_value(ins, 0, v)
}
