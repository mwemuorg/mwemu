use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let value0 = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let value1 = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    let shift_amount = (value1 & 0xFF) as u32;

    for i in 0..4 {
        let mask = 0xFFFFFFFFu128;
        let shift = i * 32;
        let dword = ((value0 >> shift) & mask) as i32;
        let shifted = dword >> shift_amount;

        result |= (shifted as u128 & mask) << shift;
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
