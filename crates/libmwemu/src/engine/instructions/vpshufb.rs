use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPSHUFB: VEX vertical op (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        let mut r = 0u128;
        for i in 0..16u32 {
            let sel = (b >> (i * 8)) & 0xff;
            if sel & 0x80 == 0 {
                let j = (sel & 0xf) as u32;
                r |= ((a >> (j * 8)) & 0xff) << (i * 8);
            }
        }
        r
    })
}
