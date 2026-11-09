use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    let s1 = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let iv = try_operand!(emu.get_operand_value(ins, 2, true));
    let f = if emu.get_operand_sz(ins, 2) == 64 {
        iv as i64 as f64
    } else {
        iv as u32 as i32 as f64
    };
    emu.set_operand_xmm_value_128(
        ins,
        0,
        (s1 & !0xffff_ffff_ffff_ffff_u128) | (f.to_bits() as u128),
    )
}
