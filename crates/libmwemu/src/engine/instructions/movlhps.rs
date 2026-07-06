use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    assert!(ins.op_count() == 2);

    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let source = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);

    let low_qword = dest & 0xFFFFFFFFFFFFFFFF;
    let high_qword = (source & 0xFFFFFFFFFFFFFFFF) << 64;
    let result = low_qword | high_qword;

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
