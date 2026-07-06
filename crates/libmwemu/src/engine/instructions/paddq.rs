use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let value0 = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let value1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);

    let mut result = 0u128;
    for i in 0..2 {
        let qword0 = ((value0 >> (64 * i)) & 0xFFFFFFFFFFFFFFFF) as u64;
        let qword1 = ((value1 >> (64 * i)) & 0xFFFFFFFFFFFFFFFF) as u64;
        let sum = qword0.wrapping_add(qword1);
        result |= (sum as u128) << (64 * i);
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
