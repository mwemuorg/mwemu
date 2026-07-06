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
    let sz = emu.get_operand_sz(ins, 0);
    let mut result: u128;

    if sz == 64 {
        result = ((value0 & 0xffff) as u16 + (value1 & 0xffff) as u16) as u128;
        result |= ((((value0 & 0xffff0000) >> 16) as u16 + ((value1 & 0xffff0000) >> 16) as u16)
            as u128)
            << 16;
        result |= ((((value0 & 0xffff00000000) >> 32) as u16
            + ((value1 & 0xffff00000000) >> 32) as u16) as u128)
            << 32;
        result |= ((((value0 & 0xffff000000000000) >> 48) as u16
            + ((value1 & 0xffff000000000000) >> 48) as u16) as u128)
            << 48;
    } else if sz == 128 {
        result = ((value0 & 0xffff) as u16 + (value1 & 0xffff) as u16) as u128;
        result |= ((((value0 & 0xffff0000) >> 16) as u16 + ((value1 & 0xffff0000) >> 16) as u16)
            as u128)
            << 16;
        result |= ((((value0 & 0xffff00000000) >> 32) as u16
            + ((value1 & 0xffff00000000) >> 32) as u16) as u128)
            << 32;
        result |= ((((value0 & 0xffff000000000000) >> 48) as u16
            + ((value1 & 0xffff000000000000) >> 48) as u16) as u128)
            << 48;

        result |= ((((value0 & 0xffff_0000000000000000) >> 64) as u16
            + ((value1 & 0xffff_0000000000000000) >> 64) as u16) as u128)
            << 64;
        result |= ((((value0 & 0xffff0000_0000000000000000) >> 80) as u16
            + ((value1 & 0xffff0000_0000000000000000) >> 80) as u16) as u128)
            << 80;
        result |= ((((value0 & 0xffff00000000_0000000000000000) >> 96) as u16
            + ((value1 & 0xffff00000000_0000000000000000) >> 96) as u16)
            as u128)
            << 96;
        result |= ((((value0 & 0xffff0000000000_0000000000000000) >> 112) as u16
            + ((value1 & 0xffff0000000000_0000000000000000) >> 112) as u16)
            as u128)
            << 112;
    } else {
        unimplemented!("bad operand size");
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
