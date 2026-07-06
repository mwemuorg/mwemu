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
        result = ((value0 & 0xff) as u8 + (value1 & 0xff) as u8) as u128;
        result |= ((((value0 & 0xff00) >> 8) as u8 + ((value1 & 0xff00) >> 8) as u8) as u128) << 8;
        result |=
            ((((value0 & 0xff0000) >> 16) as u8 + ((value1 & 0xff0000) >> 16) as u8) as u128) << 16;
        result |= ((((value0 & 0xff000000) >> 24) as u8 + ((value1 & 0xff000000) >> 24) as u8)
            as u128)
            << 24;
        result |= ((((value0 & 0xff00000000) >> 32) as u8 + ((value1 & 0xff00000000) >> 32) as u8)
            as u128)
            << 32;
        result |= ((((value0 & 0xff0000000000) >> 40) as u8
            + ((value1 & 0xff0000000000) >> 40) as u8) as u128)
            << 40;
        result |= ((((value0 & 0xff000000000000) >> 48) as u8
            + ((value1 & 0xff000000000000) >> 48) as u8) as u128)
            << 48;
        result |= ((((value0 & 0xff00000000000000) >> 56) as u8
            + ((value1 & 0xff00000000000000) >> 56) as u8) as u128)
            << 56;
    } else if sz == 128 {
        result = ((value0 & 0xff) as u8 + (value1 & 0xff) as u8) as u128;
        result |= ((((value0 & 0xff00) >> 8) as u8 + ((value1 & 0xff00) >> 8) as u8) as u128) << 8;
        result |=
            ((((value0 & 0xff0000) >> 16) as u8 + ((value1 & 0xff0000) >> 16) as u8) as u128) << 16;
        result |= ((((value0 & 0xff000000) >> 24) as u8 + ((value1 & 0xff000000) >> 24) as u8)
            as u128)
            << 24;
        result |= ((((value0 & 0xff00000000) >> 32) as u8 + ((value1 & 0xff00000000) >> 32) as u8)
            as u128)
            << 32;
        result |= ((((value0 & 0xff0000000000) >> 40) as u8
            + ((value1 & 0xff0000000000) >> 40) as u8) as u128)
            << 40;
        result |= ((((value0 & 0xff000000000000) >> 48) as u8
            + ((value1 & 0xff000000000000) >> 48) as u8) as u128)
            << 48;
        result |= ((((value0 & 0xff00000000000000) >> 56) as u8
            + ((value1 & 0xff00000000000000) >> 56) as u8) as u128)
            << 56;

        result |= ((((value0 & 0xff_0000000000000000) >> 64) as u8
            + ((value1 & 0xff_0000000000000000) >> 64) as u8) as u128)
            << 64;
        result |= ((((value0 & 0xff00_0000000000000000) >> 72) as u8
            + ((value1 & 0xff00_0000000000000000) >> 72) as u8) as u128)
            << 72;
        result |= ((((value0 & 0xff0000_0000000000000000) >> 80) as u8
            + ((value1 & 0xff0000_0000000000000000) >> 80) as u8) as u128)
            << 80;
        result |= ((((value0 & 0xff000000_0000000000000000) >> 88) as u8
            + ((value1 & 0xff000000_0000000000000000) >> 88) as u8) as u128)
            << 88;
        result |= ((((value0 & 0xff00000000_0000000000000000) >> 96) as u8
            + ((value1 & 0xff00000000_0000000000000000) >> 96) as u8) as u128)
            << 96;
        result |= ((((value0 & 0xff0000000000_0000000000000000) >> 104) as u8
            + ((value1 & 0xff0000000000_0000000000000000) >> 104) as u8)
            as u128)
            << 104;
        result |= ((((value0 & 0xff000000000000_0000000000000000) >> 112) as u8
            + ((value1 & 0xff000000000000_0000000000000000) >> 112) as u8)
            as u128)
            << 112;
        result |= ((((value0 & 0xff00000000000000_0000000000000000) >> 120) as u8
            + ((value1 & 0xff00000000000000_0000000000000000) >> 120) as u8)
            as u128)
            << 120;
    } else {
        unimplemented!("bad operand size");
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
