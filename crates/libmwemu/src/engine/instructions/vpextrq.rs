use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// VPEXTRQ r/m64, xmm, imm8[0].
pub fn execute(emu: &mut Emu, ins: &Instruction, _s: usize, _r: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let x = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    let idx = (emu.get_operand_value(ins, 2, true).unwrap_or(0) as u32) & 1;
    if !emu.set_operand_value(ins, 0, ((x >> (idx * 64)) & 0xffff_ffff_ffff_ffff) as u64) {
        return false;
    }
    true
}
