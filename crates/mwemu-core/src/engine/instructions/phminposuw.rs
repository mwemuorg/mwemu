use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let src = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let mut minv = 0xffffu32;
    let mut mini = 0u32;
    for i in 0..8u32 {
        let w = ((src >> (i * 16)) & 0xffff) as u32;
        if w < minv {
            minv = w;
            mini = i;
        }
    }
    emu.set_operand_xmm_value_128(ins, 0, (minv as u128) | ((mini as u128) << 16))
}
