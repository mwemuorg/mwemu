use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPHSUBD: VEX vertical op (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        let mut r = 0u128;
        for j in 0..2u32 {
            let x = ((a >> (64 * j)) & 0xffffffff) as u32;
            let y = ((a >> (64 * j + 32)) & 0xffffffff) as u32;
            r |= (x.wrapping_sub(y) as u128) << (32 * j);
            let p = ((b >> (64 * j)) & 0xffffffff) as u32;
            let q = ((b >> (64 * j + 32)) & 0xffffffff) as u32;
            r |= (p.wrapping_sub(q) as u128) << (32 * (j + 2));
        }
        r
    })
}
