use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VHADDPD: VEX vertical op.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        let d0 = f64::from_bits((a & 0xffff_ffff_ffff_ffff) as u64);
        let d1 = f64::from_bits(((a >> 64) & 0xffff_ffff_ffff_ffff) as u64);
        let s0 = f64::from_bits((b & 0xffff_ffff_ffff_ffff) as u64);
        let s1 = f64::from_bits(((b >> 64) & 0xffff_ffff_ffff_ffff) as u64);
        ((d0 + d1).to_bits() as u128) | (((s0 + s1).to_bits() as u128) << 64)
    })
}
