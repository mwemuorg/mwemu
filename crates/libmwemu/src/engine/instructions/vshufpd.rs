use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VSHUFPD: VEX binary op with imm8 (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop_imm(emu, ins, |a, b, imm| {
        let ga = if imm & 1 != 0 {
            (a >> 64) & 0xffff_ffff_ffff_ffff
        } else {
            a & 0xffff_ffff_ffff_ffff
        };
        let gb = if imm & 2 != 0 {
            (b >> 64) & 0xffff_ffff_ffff_ffff
        } else {
            b & 0xffff_ffff_ffff_ffff
        };
        ga | (gb << 64)
    })
}
