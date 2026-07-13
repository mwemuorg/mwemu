use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPMADDWD: VEX vertical op (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        let mut r = 0u128;
        for d in 0..4u32 {
            let a0 = ((a >> (d * 32)) & 0xffff) as u16 as i16 as i32;
            let a1 = ((a >> (d * 32 + 16)) & 0xffff) as u16 as i16 as i32;
            let b0 = ((b >> (d * 32)) & 0xffff) as u16 as i16 as i32;
            let b1 = ((b >> (d * 32 + 16)) & 0xffff) as u16 as i16 as i32;
            let w = (a0 * b0).wrapping_add(a1 * b1);
            r |= ((w as u32) as u128) << (d * 32);
        }
        r
    })
}
