use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    avx::binop_imm(emu, ins, |a, b, imm| {
        let n = (imm as u32) * 8;
        if n >= 256 {
            0
        } else if n >= 128 {
            let s = n - 128;
            if s >= 128 { 0 } else { a >> s }
        } else if n == 0 {
            b
        } else {
            (b >> n) | (a << (128 - n))
        }
    })
}
