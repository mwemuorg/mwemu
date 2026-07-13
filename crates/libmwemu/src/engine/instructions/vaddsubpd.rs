use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VADDSUBPD: VEX scalar/vertical float op.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        let mut r = 0u128;
        for i in 0..2 {
            let x = f64::from_bits(((a >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64);
            let y = f64::from_bits(((b >> (i * 64)) & 0xffff_ffff_ffff_ffff) as u64);
            let v = if i % 2 == 0 { x - y } else { x + y };
            r |= (v.to_bits() as u128) << (i * 64);
        }
        r
    })
}
