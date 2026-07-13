use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPACKSSWB: VEX vertical op (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        let s = |v: i16| -> u128 { (v.clamp(-128, 127) as i8 as u8) as u128 };
        let mut r = 0u128;
        for j in 0..8 {
            r |= s(((a >> (j * 16)) & 0xffff) as u16 as i16) << (j * 8);
            r |= s(((b >> (j * 16)) & 0xffff) as u16 as i16) << ((j + 8) * 8);
        }
        r
    })
}
