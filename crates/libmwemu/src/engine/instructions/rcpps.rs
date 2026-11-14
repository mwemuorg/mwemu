use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

// Does not model the hardware approximation; bit-exact results are not guaranteed.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let mut result = 0u128;

    for i in 0..4 {
        let shift = i * 32;
        let x = f32::from_bits(((src >> shift) & 0xffffffff) as u32);
        let r = 1.0f32 / x;
        result |= ((r.to_bits() as u128) & 0xffff_ffff) << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
