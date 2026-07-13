use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPMULDQ: VEX vertical op.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        let mut r = 0u128;
        for i in 0..2u32 {
            let x = ((a >> (i * 64)) & 0xffffffff) as u32 as i32 as i64;
            let y = ((b >> (i * 64)) & 0xffffffff) as u32 as i32 as i64;
            r |= ((x.wrapping_mul(y) as u64) as u128) << (i * 64);
        }
        r
    })
}
