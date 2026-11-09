use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Blue"), &DecodedInstruction::X86(*ins));
    let d = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    let s = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let dw = |v: u128, i: u32| ((v >> (i * 32)) & 0xffffffff) as u32;
    let w16 = (dw(d, 3) ^ dw(s, 2)).rotate_left(1);
    let w17 = (dw(d, 2) ^ dw(s, 1)).rotate_left(1);
    let w18 = (dw(d, 1) ^ dw(s, 0)).rotate_left(1);
    let w19 = (dw(d, 0) ^ w16).rotate_left(1);
    emu.set_operand_xmm_value_128(
        ins,
        0,
        (w19 as u128) | ((w18 as u128) << 32) | ((w17 as u128) << 64) | ((w16 as u128) << 96),
    )
}
