use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(color!("Cyan"), &DecodedInstruction::X86(*ins));
    let s1 = try_operand!(emu.get_operand_xmm_value_128(ins, 1, true));
    let s2 = try_operand!(emu.get_operand_xmm_value_128(ins, 2, true));
    let imm = try_operand!(emu.get_operand_value(ins, 3, true)) as u8;
    let m = if imm & 4 != 0 { 0 } else { imm & 3 };
    let x = f64::from_bits((s2 & 0xffff_ffff_ffff_ffff) as _);
    let v = match m {
        0 => x.round_ties_even(),
        1 => x.floor(),
        2 => x.ceil(),
        _ => x.trunc(),
    };
    emu.set_operand_xmm_value_128(
        ins,
        0,
        (s1 & !0xffff_ffff_ffff_ffff_u128) | (v.to_bits() as u128),
    )
}
