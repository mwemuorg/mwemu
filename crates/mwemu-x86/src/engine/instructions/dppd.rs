use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));

    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    // Immediate bits [5:4] select input lanes; bits [1:0] select output lanes.
    let imm = try_operand!(emu.get_operand_value(ins, 2, true)) as u32;
    let mut sum = 0f64;
    for i in 0..2 {
        if (imm >> (4 + i)) & 1 == 1 {
            let a = f64::from_bits(((dest >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64);
            let b = f64::from_bits(((src >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64);
            sum += a * b;
        }
    }
    let mut result = 0u128;
    for i in 0..2 {
        if (imm >> i) & 1 == 1 {
            result |= (sum.to_bits() as u128) << (i * 64);
        }
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
