use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    assert!(ins.op_count() == 3);

    let value1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let value2 = emu.get_operand_value(ins, 2, true).unwrap_or(0);

    let low_qword = value1 & 0xFFFFFFFFFFFFFFFF;
    let hw0 = (value1 >> 64) & 0xFFFF;
    let hw1 = (value1 >> 80) & 0xFFFF;
    let hw2 = (value1 >> 96) & 0xFFFF;
    let hw3 = (value1 >> 112) & 0xFFFF;
    let high_words = [hw0, hw1, hw2, hw3];
    let mut high_qword: u64 = 0;

    high_qword |= (high_words[(value2 & 0b11) as usize]) as u64;
    high_qword |= (high_words[((value2 >> 2) & 0b11) as usize] as u64) << 16;
    high_qword |= (high_words[((value2 >> 4) & 0b11) as usize] as u64) << 32;
    high_qword |= (high_words[((value2 >> 6) & 0b11) as usize] as u64) << 48;

    let result = low_qword | ((high_qword as u128) << 64);

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
