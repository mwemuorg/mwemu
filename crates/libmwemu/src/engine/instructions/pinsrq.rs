use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// PINSRQ: insert a 64-bit value from r/m into the xmm lane selected by imm8.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let s = (emu.get_operand_value(ins, 1, true).unwrap_or(0) as u128) & 0xffff_ffff_ffff_ffff;
    let idx = (emu.get_operand_value(ins, 2, true).unwrap_or(0) as u32) & 1;
    let shift = idx * 64;
    let result = (dest & !((0xffff_ffff_ffff_ffff as u128) << shift)) | (s << shift);
    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
