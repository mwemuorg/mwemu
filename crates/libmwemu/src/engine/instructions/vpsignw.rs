use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPSIGNW: VEX vertical op (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        avx::lanes(a, b, 16, |x, y| {
            let d = x as u16 as i16;
            let s = y as u16 as i16;
            (if s < 0 {
                d.wrapping_neg()
            } else if s == 0 {
                0
            } else {
                d
            }) as u16 as u128
        })
    })
}
