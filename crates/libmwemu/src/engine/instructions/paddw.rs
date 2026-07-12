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
    let lanes = match emu.get_operand_sz(ins, 0) {
        64 => 4,
        128 => 8,
        _ => unimplemented!("bad operand size"),
    };

    // Packed add of independent 16-bit lanes, each wrapping modulo 2^16.
    let mut result: u128 = 0;
    for i in 0..lanes {
        let shift = i * 16;
        let a = ((value0 >> shift) & 0xffff) as u16;
        let b = ((value1 >> shift) & 0xffff) as u16;
        result |= (a.wrapping_add(b) as u128) << shift;
    }

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
