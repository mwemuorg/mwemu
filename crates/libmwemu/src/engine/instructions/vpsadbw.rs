use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPSADBW: VEX vertical op (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        let mut r = 0u128;
        for h in 0..2u32 {
            let mut sum = 0u32;
            for k in 0..8u32 {
                let bi = h * 8 + k;
                sum += (((a >> (bi * 8)) & 0xff) as i32 - ((b >> (bi * 8)) & 0xff) as i32)
                    .unsigned_abs();
            }
            r |= (sum as u128) << (h * 64);
        }
        r
    })
}
