use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VHSUBPS: VEX vertical op.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        let mut r = 0u128;
        for j in 0..2u32 {
            let x = f32::from_bits(((a >> (64 * j)) & 0xffffffff) as u32);
            let y = f32::from_bits(((a >> (64 * j + 32)) & 0xffffffff) as u32);
            r |= ((x - y).to_bits() as u128) << (32 * j);
            let p = f32::from_bits(((b >> (64 * j)) & 0xffffffff) as u32);
            let q = f32::from_bits(((b >> (64 * j + 32)) & 0xffffffff) as u32);
            r |= ((p - q).to_bits() as u128) << (32 * (j + 2));
        }
        r
    })
}
