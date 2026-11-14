use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let mut result = 0u128;
    for i in 0..8 {
        let shift = i * 16;
        let d = ((dest >> shift) & 0xffff) as i16;
        let s = ((src >> shift) & 0xffff) as i16;
        let out = if s < 0 {
            d.wrapping_neg()
        } else if s == 0 {
            0
        } else {
            d
        };
        result |= ((out as u16) as u128) << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
