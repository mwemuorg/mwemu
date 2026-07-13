use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// INSERTPS: pick a source dword (imm8[7:6] if src is xmm, else the m32), write it
// to dest dword imm8[5:4], then zero the dwords selected by imm8[3:0].
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let imm = emu.get_operand_value(ins, 2, true).unwrap_or(0) as u8;
    let src_dword: u128 = if emu.get_operand_sz(ins, 1) == 128 {
        let s = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
        (s >> ((((imm >> 6) & 3) as u32) * 32)) & 0xffffffff
    } else {
        (emu.get_operand_value(ins, 1, true).unwrap_or(0) as u128) & 0xffffffff
    };
    let cd = ((imm >> 4) & 3) as u32;
    let mut r = (dest & !(0xffffffffu128 << (cd * 32))) | (src_dword << (cd * 32));
    for i in 0..4u32 {
        if (imm >> i) & 1 == 1 {
            r &= !(0xffffffffu128 << (i * 32));
        }
    }
    emu.set_operand_xmm_value_128(ins, 0, r);
    true
}
