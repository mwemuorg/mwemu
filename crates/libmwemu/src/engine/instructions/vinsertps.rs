use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VINSERTPS: VEX op.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop_imm(emu, ins, |a, b, imm| {
        let sd = (b >> ((((imm >> 6) & 3) as u32) * 32)) & 0xffffffff;
        let cd = ((imm >> 4) & 3) as u32;
        let mut r = (a & !(0xffffffffu128 << (cd * 32))) | (sd << (cd * 32));
        for i in 0..4u32 {
            if (imm >> i) & 1 == 1 {
                r &= !(0xffffffffu128 << (i * 32));
            }
        }
        r
    })
}
