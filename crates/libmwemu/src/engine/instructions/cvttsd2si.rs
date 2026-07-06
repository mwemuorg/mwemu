use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// CVTTSD2SI r32/r64, xmm/m64 : convert a scalar double to a signed integer
// with truncation (toward zero).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let dst_sz = emu.get_operand_sz(ins, 0);
    let src = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };
    let f = f64::from_bits((src & 0xFFFFFFFFFFFFFFFF) as u64);

    let result: u64 = if dst_sz == 64 {
        if f.is_nan() || f >= 9223372036854775808.0 || f < -9223372036854775808.0 {
            0x8000000000000000
        } else {
            (f.trunc() as i64) as u64
        }
    } else if f.is_nan() || f >= 2147483648.0 || f < -2147483648.0 {
        0x80000000
    } else {
        ((f.trunc() as i32) as u32) as u64
    };

    emu.set_operand_value(ins, 0, result);
    true
}
