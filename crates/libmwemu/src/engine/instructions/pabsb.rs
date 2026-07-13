use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PABSB: per-byte absolute value (signed) of the source into the destination.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for i in 0..16 {
        let b = ((src >> (i * 8)) & 0xff) as u8 as i8;
        result |= (b.unsigned_abs() as u128) << (i * 8);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
