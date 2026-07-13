use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// PUNPCKHQDQ: interleave the high qwords: result = [dest.hi64, src.hi64].
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let lo = (dest >> 64) & 0xffff_ffff_ffff_ffff;
    let hi = (src >> 64) & 0xffff_ffff_ffff_ffff;
    emu.set_operand_xmm_value_128(ins, 0, lo | (hi << 64));
    true
}
