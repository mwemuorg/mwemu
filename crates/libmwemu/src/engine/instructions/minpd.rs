use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let mut result = 0u128;

    for i in 0..2 {
        let shift = i * 64;
        let a = f64::from_bits(((dest >> shift) & 0xffff_ffff_ffff_ffff) as _);
        let b = f64::from_bits(((src >> shift) & 0xffff_ffff_ffff_ffff) as _);
        let r: f64 = if a < b { a } else { b };
        result |= ((r.to_bits() as u128) & 0xffff_ffff_ffff_ffff_u128) << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
