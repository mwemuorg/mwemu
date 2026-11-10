use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    let s = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let mut m = 0u64;
    for i in 0..2 {
        if (s >> (i * 64 + 64 - 1)) & 1 == 1 {
            m |= 1 << i;
        }
    }
    emu.set_operand_value(ins, 0, m)
}
