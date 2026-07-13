use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// SHA1MSG2: message schedule using ROL1.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let d = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let s = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let dw = |v: u128, i: u32| ((v >> (i * 32)) & 0xffffffff) as u32;
    let w16 = (dw(d, 3) ^ dw(s, 2)).rotate_left(1);
    let w17 = (dw(d, 2) ^ dw(s, 1)).rotate_left(1);
    let w18 = (dw(d, 1) ^ dw(s, 0)).rotate_left(1);
    let w19 = (dw(d, 0) ^ w16).rotate_left(1);
    emu.set_operand_xmm_value_128(
        ins,
        0,
        (w19 as u128) | ((w18 as u128) << 32) | ((w17 as u128) << 64) | ((w16 as u128) << 96),
    );
    true
}
