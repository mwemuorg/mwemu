use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let mut result = 0u128;
    for i in 0..2 {
        let a = ((dest >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64 as i64;
        let b = ((src >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64 as i64;
        let lane: u128 = if a > b { 0xffff_ffff_ffff_ffff } else { 0 };
        result |= lane << (i * 64);
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
