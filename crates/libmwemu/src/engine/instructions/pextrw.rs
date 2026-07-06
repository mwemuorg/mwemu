use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

/// `PEXTRW r32/m16, xmm, imm8` — extract the 16-bit word selected by `imm8 & 7`
/// from the source xmm and store it (zero-extended) into the destination.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let source = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => {
            log::trace!("pextrw: error getting source xmm");
            return false;
        }
    };
    let sel = match emu.get_operand_value(ins, 2, true) {
        Some(v) => v & 0x7,
        None => {
            log::trace!("pextrw: error getting selector");
            return false;
        }
    };

    let word = ((source >> (sel * 16)) & 0xffff) as u64;
    emu.set_operand_value(ins, 0, word);
    true
}
