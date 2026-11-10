use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let mut result = dest & !0xffffffff_u128;
    for i in 0..1 {
        let shift = i * 32;
        let a = f32::from_bits(((dest >> shift) & 0xffffffff) as _);
        let b = f32::from_bits(((src >> shift) & 0xffffffff) as _);
        let r: f32 = if a > b { a } else { b };
        result |= ((r.to_bits() as u128) & 0xffffffff_u128) << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
