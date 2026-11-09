use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));

    let dest = try_operand!(emu.get_operand_xmm_value_128(ins, 0, true));
    // Bits [7:6] select a register source lane; memory sources use their only dword.
    // Bits [5:4] select the destination; bits [3:0] zero lanes after insertion.
    let imm = try_operand!(emu.get_operand_value(ins, 2, true)) as u8;
    let src_dword: u128 = if emu.get_operand_sz(ins, 1) == 128 {
        let s = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
        (s >> ((((imm >> 6) & 3) as u32) * 32)) & 0xffffffff
    } else {
        (try_operand!(emu.get_operand_value(ins, 1, true)) as u128) & 0xffffffff
    };
    let cd = ((imm >> 4) & 3) as u32;
    let mut r = (dest & !(0xffffffffu128 << (cd * 32))) | (src_dword << (cd * 32));
    for i in 0..4u32 {
        if (imm >> i) & 1 == 1 {
            r &= !(0xffffffffu128 << (i * 32));
        }
    }
    emu.set_operand_xmm_value_128(ins, 0, r)
}
