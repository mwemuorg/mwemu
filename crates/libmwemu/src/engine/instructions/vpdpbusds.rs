use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPDPBUSDS: VNNI dot-product accumulate.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::ternop_acc(emu, ins, |d, a, b| {
        let mut r = d;
        for i in 0..4u32 {
            let mut acc = ((d >> (i * 32)) & 0xffffffff) as u32 as i32 as i64;
            for k in 0..4u32 {
                let u = ((a >> (i * 32 + k * 8)) & 0xff) as u8 as i64;
                let s = ((b >> (i * 32 + k * 8)) & 0xff) as u8 as i8 as i64;
                acc += u * s;
            }
            let sat = acc.clamp(i32::MIN as i64, i32::MAX as i64) as i32;
            r = (r & !(0xffffffffu128 << (i * 32))) | (((sat as u32) as u128) << (i * 32));
        }
        r
    })
}
