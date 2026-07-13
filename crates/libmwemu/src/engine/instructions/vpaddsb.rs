use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPADDSB: VEX vertical op.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        avx::lanes(a, b, 8, |x, y| {
            ((x as u8 as i8).saturating_add(y as u8 as i8) as u8) as u128
        })
    })
}
