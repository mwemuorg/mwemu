use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPBLENDD: VEX op with imm8.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop_imm(emu, ins, |a, b, imm| {
        let mut r = 0u128;
        for i in 0..4u32 {
            let sh = 32 * i;
            r |= if (imm >> i) & 1 == 1 {
                (b >> sh) & 0xffffffff
            } else {
                (a >> sh) & 0xffffffff
            } << sh;
        }
        r
    })
}
