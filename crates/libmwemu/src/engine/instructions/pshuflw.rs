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

    let high_qword = value1 & 0xFFFFFFFFFFFFFFFF_0000000000000000;
    let lw0 = value1 & 0xFFFF;
    let lw1 = (value1 >> 16) & 0xFFFF;
    let lw2 = (value1 >> 32) & 0xFFFF;
    let lw3 = (value1 >> 48) & 0xFFFF;
    let low_words = [lw0, lw1, lw2, lw3];
    let mut low_qword: u64 = 0;
    low_qword |= (low_words[(value2 & 0b11) as usize]) as u64;
    low_qword |= (low_words[((value2 >> 2) & 0b11) as usize] as u64) << 16;
    low_qword |= (low_words[((value2 >> 4) & 0b11) as usize] as u64) << 32;
    low_qword |= (low_words[((value2 >> 6) & 0b11) as usize] as u64) << 48;
    let result = high_qword | low_qword as u128;

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
