use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    if emu.get_operand_sz(ins, 1) < 128 {
        let value = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);

        let shift_amount = match emu.get_operand_value(ins, 1, false) {
            Some(v) => (v & 0xFF) as u32,
            None => 0,
        };

        let mut result = 0u128;

        for i in 0..8 {
            let mask = 0xFFFFu128;
            let shift = i * 16;
            let word = ((value >> shift) & mask) as u16;
            let shifted_word = (word as u32 >> shift_amount) as u16;

            result |= (shifted_word as u128) << shift;
        }

        emu.set_operand_xmm_value_128(ins, 0, result);
    } else {
        let value = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);

        let shift_amount = match emu.get_operand_xmm_value_128(ins, 1, false) {
            Some(v) => (v & 0xFF) as u32,
            None => 0,
        };

        let mut result = 0u128;

        for i in 0..8 {
            let mask = 0xFFFFu128;
            let shift = i * 16;
            let word = ((value >> shift) & mask) as u16;
            let shifted_word = (word as u32 >> shift_amount) as u16;

            result |= (shifted_word as u128) << shift;
        }

        emu.set_operand_xmm_value_128(ins, 0, result);
    }
    true
}
