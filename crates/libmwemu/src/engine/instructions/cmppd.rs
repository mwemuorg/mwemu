use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// CMPPD: per-lane f64 compare selected by imm8[2:0]; each lane becomes
// all-ones (true) or zero (false).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let pred = emu.get_operand_value(ins, 2, true).unwrap_or(0) as u8 & 7;
    let mut result = 0u128;

    for i in 0..2 {
        let shift = i * 64;
        let a = f64::from_bits(((dest >> shift) & 0xffff_ffff_ffff_ffff) as _);
        let b = f64::from_bits(((src >> shift) & 0xffff_ffff_ffff_ffff) as _);
        let t = match pred {
            0 => a == b,
            1 => a < b,
            2 => a <= b,
            3 => a.is_nan() || b.is_nan(),
            4 => !(a == b),
            5 => !(a < b),
            6 => !(a <= b),
            _ => !a.is_nan() && !b.is_nan(),
        };
        if t {
            result |= (0xffff_ffff_ffff_ffff as u128) << shift;
        }
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
