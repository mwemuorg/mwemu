use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VROUNDPS: VEX unary op with imm8 (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::unop_imm(emu, ins, |a, imm| {
        let m = if imm & 4 != 0 { 0 } else { imm & 3 };
        let mut r = 0u128;
        for i in 0..4u32 {
            let x = f32::from_bits(((a >> (i * 32)) & 0xffffffff) as u32);
            let v = match m {
                0 => x.round_ties_even(),
                1 => x.floor(),
                2 => x.ceil(),
                _ => x.trunc(),
            };
            r |= (v.to_bits() as u128) << (i * 32);
        }
        r
    })
}
