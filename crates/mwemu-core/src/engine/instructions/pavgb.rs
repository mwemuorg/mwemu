use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let mut result = 0u128;
    for i in 0..16 {
        let shift = i * 8;
        let a = ((dest >> shift) & 0xff) as u16;
        let b = ((src >> shift) & 0xff) as u16;
        result |= (((a + b + 1) >> 1) as u128 & 0xff) << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
