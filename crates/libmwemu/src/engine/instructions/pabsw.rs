use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PABSW: per-word (16-bit) absolute value (signed).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for i in 0..8 {
        let w = ((src >> (i * 16)) & 0xffff) as u16 as i16;
        result |= (w.unsigned_abs() as u128) << (i * 16);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
