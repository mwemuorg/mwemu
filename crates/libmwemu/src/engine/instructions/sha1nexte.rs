use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// SHA1NEXTE: add ROL30 of the E word into SRC's high dword.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Blue"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let d = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let s = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let dw = |v: u128, i: u32| ((v >> (i * 32)) & 0xffffffff) as u32;
    let tmp = dw(d, 3).rotate_left(30);
    let r3 = dw(s, 3).wrapping_add(tmp);
    emu.set_operand_xmm_value_128(
        ins,
        0,
        (dw(s, 0) as u128)
            | ((dw(s, 1) as u128) << 32)
            | ((dw(s, 2) as u128) << 64)
            | ((r3 as u128) << 96),
    );
    true
}
