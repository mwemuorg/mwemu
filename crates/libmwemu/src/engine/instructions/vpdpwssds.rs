use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPDPWSSDS: VNNI dot-product accumulate.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::ternop_acc(emu, ins, |d, a, b| {
        let mut r = d;
        for i in 0..4u32 {
            let mut acc = ((d >> (i * 32)) & 0xffffffff) as u32 as i32 as i64;
            for k in 0..2u32 {
                let x = ((a >> (i * 32 + k * 16)) & 0xffff) as u16 as i16 as i64;
                let y = ((b >> (i * 32 + k * 16)) & 0xffff) as u16 as i16 as i64;
                acc += x * y;
            }
            let sat = acc.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
            r = (r & !(0xffffffffu128 << (i * 32))) | (((sat as u32) as u128) << (i * 32));
        }
        r
    })
}
