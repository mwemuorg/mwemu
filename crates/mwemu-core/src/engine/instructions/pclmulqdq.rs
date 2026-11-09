use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Blue"), &DecodedInstruction::X86(*ins));

    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    // Immediate bits 0 and 4 select the destination and source qwords.
    let imm = try_operand!(emu.get_operand_value(ins, 2, true));
    let a = ((dest >> (if imm & 1 != 0 { 64 } else { 0 })) & 0xffff_ffff_ffff_ffff) as u64;
    let b = ((src >> (if imm & 0x10 != 0 { 64 } else { 0 })) & 0xffff_ffff_ffff_ffff) as u64;
    let mut result = 0u128;
    for i in 0..64 {
        if (b >> i) & 1 == 1 {
            result ^= (a as u128) << i;
        }
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
