use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PHADDD: horizontal add of adjacent 32-bit lane pairs.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for j in 0..2 {
        let a = ((dest >> (64 * j)) & 0xffffffff) as u32;
        let b = ((dest >> (64 * j + 32)) & 0xffffffff) as u32;
        result |= (a.wrapping_add(b) as u128) << (32 * j);
        let c = ((src >> (64 * j)) & 0xffffffff) as u32;
        let d = ((src >> (64 * j + 32)) & 0xffffffff) as u32;
        result |= (c.wrapping_add(d) as u128) << (32 * (j + 2));
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
