use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPSRAVD: VEX op.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        avx::lanes(a, b, 32, |x, y| {
            let c = y as u32;
            if c >= 32 {
                if (x >> 31) & 1 == 1 { 0xffffffff } else { 0 }
            } else {
                ((x as u32 as i32) >> c) as u32 as u128
            }
        })
    })
}
