use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;
// VCVTDQ2PS: packed int32 -> f32.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::unop(emu, ins, |a| {
        let mut r = 0u128;
        for i in 0..4 {
            let d = ((a >> (i * 32)) & 0xffffffff) as u32 as i32;
            r |= ((d as f32).to_bits() as u128) << (i * 32);
        }
        r
    })
}
