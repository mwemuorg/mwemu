use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// UNPCKHPS: interleave high dwords -> [d2, s2, d3, s3].
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let d = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let s = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let g = |v: u128, n: u32| (v >> (32 * n)) & 0xffffffff;
    let r = g(d, 2) | (g(s, 2) << 32) | (g(d, 3) << 64) | (g(s, 3) << 96);
    emu.set_operand_xmm_value_128(ins, 0, r);
    true
}
