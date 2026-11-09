use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    avx::binop_imm(emu, ins, |a, b, imm| {
        let mut r = 0u128;
        for i in 0..2u32 {
            let x = f64::from_bits(((a >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64);
            let y = f64::from_bits(((b >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64);
            if avx::cmp_pred_f64(x, y, imm) {
                r |= (0xffff_ffff_ffff_ffffu128) << (i * 64);
            }
        }
        r
    })
}
