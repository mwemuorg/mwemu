use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;
// VCMPPS dest, src1, src2, imm8: per-f32-lane compare -> all-ones/0.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop_imm(emu, ins, |a, b, imm| {
        let mut r = 0u128;
        for i in 0..4u32 {
            let x = f32::from_bits(((a >> (i * 32)) & 0xffffffff) as u32);
            let y = f32::from_bits(((b >> (i * 32)) & 0xffffffff) as u32);
            if avx::cmp_pred_f32(x, y, imm) {
                r |= 0xffffffffu128 << (i * 32);
            }
        }
        r
    })
}
