use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let value0 = match emu.get_operand_xmm_value_128(ins, 0, true) {
        Some(v) => v,
        None => {
            log::trace!("error getting value0");
            return false;
        }
    };
    let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => {
            log::trace!("error getting value1");
            return false;
        }
    };
    let mut result: u128;

    if value1 > 15 {
        result = value0 & 0xffffffffffffffff_0000000000000000;
    } else {
        result = (((value0 & 0xffff) as u16) << value1) as u128;
        result |= (((((value0 & 0xffff0000) >> 16) as u16) << value1) as u128) << 16;
        result |= (((((value0 & 0xffff00000000) >> 32) as u16) << value1) as u128) << 32;
        result |= (((((value0 & 0xffff000000000000) >> 48) as u16) << value1) as u128) << 48;
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
