use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPMADDUBSW: VEX vertical op (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        let mut r = 0u128;
        for j in 0..8u32 {
            let du0 = ((a >> (16 * j)) & 0xff) as u8 as i32;
            let du1 = ((a >> (16 * j + 8)) & 0xff) as u8 as i32;
            let ss0 = ((b >> (16 * j)) & 0xff) as u8 as i8 as i32;
            let ss1 = ((b >> (16 * j + 8)) & 0xff) as u8 as i8 as i32;
            let s = (du0 * ss0 + du1 * ss1).clamp(-32768, 32767) as i16;
            r |= ((s as u16) as u128) << (16 * j);
        }
        r
    })
}
