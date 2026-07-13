use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// MINPS: per-lane f32 operation.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;

    for i in 0..4 {
        let shift = i * 32;
        let a = f32::from_bits(((dest >> shift) & 0xffffffff) as _);
        let b = f32::from_bits(((src >> shift) & 0xffffffff) as _);
        let r: f32 = if a < b { a } else { b };
        result |= ((r.to_bits() as u128) & (0xffffffff as u128)) << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
