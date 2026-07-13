use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// MOVSHDUP: duplicate odd-indexed dwords -> [s1,s1,s3,s3].
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let s = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let g = |n: u32| (s >> (32 * n)) & 0xffffffff;
    emu.set_operand_xmm_value_128(ins, 0, g(1) | (g(1) << 32) | (g(3) << 64) | (g(3) << 96));
    true
}
