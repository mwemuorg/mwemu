use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPHSUBSW: VEX vertical op (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        let mut r = 0u128;
        for j in 0..4u32 {
            let x = ((a >> (32 * j)) & 0xffff) as u16 as i16;
            let y = ((a >> (32 * j + 16)) & 0xffff) as u16 as i16;
            r |= ((x.saturating_sub(y) as u16) as u128) << (16 * j);
            let p = ((b >> (32 * j)) & 0xffff) as u16 as i16;
            let q = ((b >> (32 * j + 16)) & 0xffff) as u16 as i16;
            r |= ((p.saturating_sub(q) as u16) as u128) << (16 * (j + 4));
        }
        r
    })
}
