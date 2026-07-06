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

    result = (value0 & 0xffffffff) as u32 as i32 as i16 as u16 as u128;
    result |= (((value0 & 0xffffffff00000000) >> 32) as u32 as i32 as i16 as u16 as u128) << 16;
    result |=
        (((value0 & 0xffffffff0000000000000000) >> 64) as u32 as i32 as i16 as u16 as u128) << 32;
    result |= (((value0 & 0xffffffff000000000000000000000000) >> 96) as u32 as i32 as i16 as u16
        as u128)
        << 48;
    result |= ((value1 & 0xffffffff) as u32 as i32 as i16 as u16 as u128) << 64;
    result |= (((value1 & 0xffffffff00000000) >> 32) as u32 as i32 as i16 as u16 as u128) << 80;
    result |=
        (((value1 & 0xffffffff0000000000000000) >> 64) as u32 as i32 as i16 as u16 as u128) << 96;
    result |= (((value1 & 0xffffffff000000000000000000000000) >> 96) as u32 as i32 as i16 as u16
        as u128)
        << 112;

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
