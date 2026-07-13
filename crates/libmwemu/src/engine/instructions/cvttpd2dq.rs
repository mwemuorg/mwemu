use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// CVTTPD2DQ: 2 packed f64 -> low 2 packed int32 (high 64 bits zeroed).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let mut result = 0u128;
    for i in 0..2 {
        let f = f64::from_bits(((src >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64);
        let r = if true { f.trunc() } else { f.round_ties_even() };
        let v: i32 = if r.is_nan() || r >= 2147483648.0 || r < -2147483648.0 {
            i32::MIN
        } else {
            r as i32
        };
        result |= ((v as u32) as u128) << (i * 32);
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
