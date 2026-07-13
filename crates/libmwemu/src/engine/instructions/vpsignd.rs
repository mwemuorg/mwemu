use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPSIGND: VEX vertical op (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        avx::lanes(a, b, 32, |x, y| {
            let d = x as u32 as i32;
            let s = y as u32 as i32;
            (if s < 0 {
                d.wrapping_neg()
            } else if s == 0 {
                0
            } else {
                d
            }) as u32 as u128
        })
    })
}
