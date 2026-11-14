use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Red"), &DecodedInstruction::X86(*ins));
    debug_assert!(ins.op_count() == 2);

    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let source = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));

    let low_qword = dest & 0xFFFFFFFFFFFFFFFF;
    let high_qword = (source & 0xFFFFFFFFFFFFFFFF) << 64;
    let result = low_qword | high_qword;

    emu.set_operand_xmm_value_128(ins, 0, result)
}
