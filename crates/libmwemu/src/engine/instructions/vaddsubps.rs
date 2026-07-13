use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VADDSUBPS: VEX scalar/vertical float op.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        let mut r = 0u128;
        for i in 0..4 {
            let x = f32::from_bits(((a >> (i * 32)) & 0xffffffff) as u32);
            let y = f32::from_bits(((b >> (i * 32)) & 0xffffffff) as u32);
            let v = if i % 2 == 0 { x - y } else { x + y };
            r |= (v.to_bits() as u128) << (i * 32);
        }
        r
    })
}
