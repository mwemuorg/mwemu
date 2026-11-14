use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::engine::instructions::avx;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let (a0, a1) = avx::to_pair(try_operand!(emu.get_operand_ymm_value_256(ins, 1, true)));
    let (b0, b1) = avx::to_pair(try_operand!(emu.get_operand_ymm_value_256(ins, 2, true)));
    let imm = try_operand!(emu.get_operand_value(ins, 3, true)) as u32;
    let sel = |s: u32| -> u128 {
        match s & 3 {
            0 => a0,
            1 => a1,
            2 => b0,
            _ => b1,
        }
    };
    // Bits 3 and 7 independently zero the low and high result halves.
    let lo = if imm & 8 != 0 { 0 } else { sel(imm & 3) };
    let hi = if imm & 0x80 != 0 {
        0
    } else {
        sel((imm >> 4) & 3)
    };
    emu.set_operand_ymm_value_256(ins, 0, avx::from_pair(lo, hi))
}
