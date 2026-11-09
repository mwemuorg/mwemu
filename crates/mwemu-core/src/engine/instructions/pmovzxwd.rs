use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let mut result = 0u128;
    for i in 0..4 {
        let raw = (src >> (i * 16)) & ((1u128 << 16) - 1);
        let ext = raw;
        result |= (ext & ((1u128 << 32) - 1)) << (i * 32);
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
