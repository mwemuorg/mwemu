use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PCMPEQQ: packed 64-bit equality; each lane becomes all-ones or all-zeros.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for i in 0..2 {
        let a = (dest >> (i * 64)) & 0xffff_ffff_ffff_ffff;
        let b = (src >> (i * 64)) & 0xffff_ffff_ffff_ffff;
        let lane: u128 = if a == b { 0xffff_ffff_ffff_ffff } else { 0 };
        result |= lane << (i * 64);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
