use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VGF2P8AFFINEINVQB: VEX op with imm8.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop_imm(emu, ins, |a, b, imm| {
        let mut r = 0u128;
        for j in 0..16u32 {
            let x = crate::engine::instructions::aes::gf_inv(((a >> (j * 8)) & 0xff) as u8);
            let blk = j / 8;
            let mut o = 0u8;
            for i in 0..8u32 {
                let row = ((b >> ((blk * 8 + (7 - i)) * 8)) & 0xff) as u8;
                let bit = ((row & x).count_ones() & 1) as u8 ^ ((imm >> i) & 1);
                o |= bit << i;
            }
            r |= (o as u128) << (j * 8);
        }
        r
    })
}
