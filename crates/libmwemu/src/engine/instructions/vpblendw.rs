use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPBLENDW: VEX binary op with imm8 (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop_imm(emu, ins, |a, b, imm| {
        let mut r = 0u128;
        for i in 0..8u32 {
            let sh = 16 * i;
            r |= if (imm >> i) & 1 == 1 {
                (b >> sh) & 0xffff
            } else {
                (a >> sh) & 0xffff
            } << sh;
        }
        r
    })
}
