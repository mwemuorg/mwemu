use crate::color;
use crate::emu::Emu;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

// VPACKSSDW: VEX vertical op (128/256).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    avx::binop(emu, ins, |a, b| {
        let s = |v: i32| -> u128 { (v.clamp(-32768, 32767) as i16 as u16) as u128 };
        let mut r = 0u128;
        for j in 0..4 {
            r |= s(((a >> (j * 32)) & 0xffffffff) as u32 as i32) << (j * 16);
            r |= s(((b >> (j * 32)) & 0xffffffff) as u32 as i32) << ((j + 4) * 16);
        }
        r
    })
}
