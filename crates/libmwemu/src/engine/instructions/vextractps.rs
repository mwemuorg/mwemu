use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// VEXTRACTPS r/m32, xmm, imm8[1:0]: extract a dword.
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let x = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let idx = (emu.get_operand_value(ins, 2, true).unwrap_or(0) as u32) & 3;
    if !emu.set_operand_value(ins, 0, ((x >> (idx * 32)) & 0xffffffff) as u64) {
        return false;
    }
    true
}
