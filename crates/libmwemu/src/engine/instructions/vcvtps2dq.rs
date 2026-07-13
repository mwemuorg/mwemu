use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;
// VCVTPS2DQ: packed f32 -> int32.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::unop(emu, ins, |a| {
        let mut r = 0u128;
        for i in 0..4u32 {
            let x = f32::from_bits(((a >> (i * 32)) & 0xffffffff) as u32);
            let v = if false {
                x.trunc()
            } else {
                x.round_ties_even()
            };
            let d: i32 = if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 {
                i32::MIN
            } else {
                v as i32
            };
            r |= ((d as u32) as u128) << (i * 32);
        }
        r
    })
}
