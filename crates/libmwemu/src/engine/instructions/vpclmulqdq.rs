use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Blue"), &DecodedInstruction::X86(*ins));
    let s1 = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let s2 = try_operand!(emu.get_operand_xmm_value_128(ins, 2, true));
    let imm = try_operand!(emu.get_operand_value(ins, 3, true));
    let a = ((s1 >> (if imm & 1 != 0 { 64 } else { 0 })) & 0xffff_ffff_ffff_ffff) as u64;
    let b = ((s2 >> (if imm & 0x10 != 0 { 64 } else { 0 })) & 0xffff_ffff_ffff_ffff) as u64;
    let mut r = 0u128;
    for i in 0..64 {
        if (b >> i) & 1 == 1 {
            r ^= (a as u128) << i;
        }
    }
    emu.set_operand_xmm_value_128(ins, 0, r)
}
