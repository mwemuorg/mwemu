use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;
// UNPCKLPD: [dest.lo64, src.lo64].
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let d = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let s = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    emu.set_operand_xmm_value_128(
        ins,
        0,
        (d & 0xffff_ffff_ffff_ffff) | ((s & 0xffff_ffff_ffff_ffff) << 64),
    );
    true
}
