use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let imm = try_operand!(emu.get_operand_value(ins, 2, true));
    let mut result = 0u128;
    for i in 0..4 {
        let shift = i * 32;
        let lane = if (imm >> i) & 1 == 1 {
            (src >> shift) & 0xffffffff
        } else {
            (dest >> shift) & 0xffffffff
        };
        result |= lane << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
