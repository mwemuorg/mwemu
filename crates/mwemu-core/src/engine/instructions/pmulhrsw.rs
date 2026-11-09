use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let mut result = 0u128;
    for j in 0..8 {
        let shift = 16 * j;
        let d = ((dest >> shift) & 0xffff) as u16 as i16 as i32;
        let s = ((src >> shift) & 0xffff) as u16 as i16 as i32;
        // Round the fixed-point product before extracting bits [30:15].
        let res = (((d * s) >> 14) + 1) >> 1;
        result |= ((res as u16) as u128) << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
