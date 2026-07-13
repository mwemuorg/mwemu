use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PHADDSW: horizontal saturating 16-bit saturating of adjacent lane pairs.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for j in 0..4 {
        let a = ((dest >> (32 * j)) & 0xffff) as u16 as i16;
        let b = ((dest >> (32 * j + 16)) & 0xffff) as u16 as i16;
        result |= ((a.saturating_add(b) as u16) as u128) << (16 * j);
        let c = ((src >> (32 * j)) & 0xffff) as u16 as i16;
        let d = ((src >> (32 * j + 16)) & 0xffff) as u16 as i16;
        result |= ((c.saturating_add(d) as u16) as u128) << (16 * (j + 4));
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
