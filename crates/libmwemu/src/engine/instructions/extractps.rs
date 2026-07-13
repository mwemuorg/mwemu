use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// EXTRACTPS: extract dword imm8[1:0] of the xmm source into r/m32.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let x = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let idx = (emu.get_operand_value(ins, 2, true).unwrap_or(0) as u32) & 3;
    let v = ((x >> (idx * 32)) & 0xffffffff) as u64;
    if !emu.set_operand_value(ins, 0, v) {
        return false;
    }
    true
}
