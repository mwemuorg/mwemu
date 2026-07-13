use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PMINSD: packed 32-bit min (i32).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for i in 0..4 {
        let a = ((dest >> (i * 32)) & 0xffffffff) as u32 as i32;
        let b = ((src >> (i * 32)) & 0xffffffff) as u32 as i32;
        result |= ((a.min(b) as u32) as u128) << (i * 32);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
