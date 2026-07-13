use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// ROUNDPD: round f64 lanes per imm8 (bit2=MXCSR mode; bits[1:0]: 0=nearest,1=down,2=up,3=trunc).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let imm = emu.get_operand_value(ins, 2, true).unwrap_or(0) as u8;
    let mode = if imm & 4 != 0 { 0 } else { imm & 3 };
    let mut result = 0u128;

    for i in 0..2 {
        let shift = i * 64;
        let x = f64::from_bits(((src >> shift) & 0xffff_ffff_ffff_ffff) as _);
        let r = match mode {
            0 => x.round_ties_even(),
            1 => x.floor(),
            2 => x.ceil(),
            _ => x.trunc(),
        };
        result |= ((r.to_bits() as u128) & (0xffff_ffff_ffff_ffff as u128)) << shift;
    }
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
