use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PMULHRSW: signed 16-bit multiply, take bits [30:15] of the product with
// rounding: (((d * s) >> 14) + 1) >> 1.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for j in 0..8 {
        let shift = 16 * j;
        let d = ((dest >> shift) & 0xffff) as u16 as i16 as i32;
        let s = ((src >> shift) & 0xffff) as u16 as i16 as i32;
        let res = (((d * s) >> 14) + 1) >> 1;
        result |= ((res as u16) as u128) << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
