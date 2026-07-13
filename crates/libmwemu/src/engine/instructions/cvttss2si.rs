use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// CVTTSS2SI: low f32 -> signed integer GPR (truncating).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let f = f32::from_bits((src & 0xffffffff) as u32);
    let r = if true { f.trunc() } else { f.round_ties_even() };
    let result: u64 = if emu.get_operand_sz(ins, 0) == 64 {
        if r.is_nan() || r >= 9223372036854775808.0 || r < -9223372036854775808.0 {
            i64::MIN as u64
        } else {
            r as i64 as u64
        }
    } else if r.is_nan() || r >= 2147483648.0 || r < -2147483648.0 {
        i32::MIN as u32 as u64
    } else {
        r as i32 as u32 as u64
    };
    if !emu.set_operand_value(ins, 0, result) {
        return false;
    }
    true
}
