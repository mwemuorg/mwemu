use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let mut result = 0u128;
    for i in 0..2 {
        let raw = (src >> (i * 8)) & ((1u128 << 8) - 1);
        let sign = 1u128 << (8 - 1);
        let ext = if raw & sign != 0 {
            raw | !((1u128 << 8) - 1)
        } else {
            raw
        };
        result |= (ext & (1u128 << 64).wrapping_sub(1)) << (i * 64);
    }
    emu.set_operand_xmm_value_128(ins, 0, result)
}
