use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

// ANDNPD: dest = (NOT dest) AND src (128-bit bitwise).
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let dest = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let src = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0);
    emu.set_operand_xmm_value_128(ins, 0, (!dest) & src);
    true
}
