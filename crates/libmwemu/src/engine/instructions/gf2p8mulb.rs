use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::aes;
use iced_x86::Instruction;
// GF2P8MULB: per-byte multiply in GF(2^8) with the AES reduction polynomial.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for i in 0..16 {
        let a = ((dest >> (i * 8)) & 0xff) as u8;
        let b = ((src >> (i * 8)) & 0xff) as u8;
        result |= (aes::gmul(a, b) as u128) << (i * 8);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
