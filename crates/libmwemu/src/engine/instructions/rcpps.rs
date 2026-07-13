use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// RCPPS: approximate recip of f32 lanes (implemented exactly; hardware uses
// a ~12-bit approximation, so bit-exact match with silicon is not expected).
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
        let x = f32::from_bits(((src >> shift) & 0xffffffff) as u32);
        let r = 1.0f32 / x;
        result |= ((r.to_bits() as u128) & 0xffff_ffff) << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
