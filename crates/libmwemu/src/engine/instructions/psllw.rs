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
    // Shift each 16-bit lane left by the count in the low bits of operand 1.
    // A count >= 16 shifts every lane fully out, so the whole result is zero.
    let lanes = if emu.get_operand_sz(ins, 0) == 128 {
        8
    } else {
        4
    };
    let count = value1 as u64; // only the low 64 bits are the shift count
    let mut result: u128 = 0;
    if count < 16 {
        for i in 0..lanes {
            let shift = i * 16;
            let w = ((value0 >> shift) & 0xffff) as u16;
            result |= ((w.wrapping_shl(count as u32) as u128) & 0xffff) << shift;
        }
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
