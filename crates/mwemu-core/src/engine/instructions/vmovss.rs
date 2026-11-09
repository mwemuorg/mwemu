use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    if ins.op_count() == 3 {
        let s1 = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
        let s2 = try_operand!(emu.get_operand_xmm_value_128(ins, 2, true));
        emu.set_operand_xmm_value_128(ins, 0, (s1 & !0xffffffff_u128) | (s2 & 0xffffffff))
    } else {
        let s = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
        emu.set_operand_xmm_value_128(ins, 0, s & 0xffffffff)
    }
}
