use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PMULHUW: packed 16-bit lane operation.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for i in 0..8 {
        let shift = i * 16;
        let a = ((dest >> shift) & 0xffff) as u32;
        let b = ((src >> shift) & 0xffff) as u32;
        result |= (((a * b) >> 16) as u128 & 0xffff) << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
