use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

/// `PINSRW xmm, r32/m16, imm8` — insert the low 16 bits of the source into the
/// word position selected by `imm8 & 7` of the destination xmm, leaving the
/// other seven words unchanged.
pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let mut dest = match emu.get_operand_xmm_value_128(ins, 0, true) {
        Some(v) => v,
        None => {
            log::trace!("pinsrw: error getting destination xmm");
            return false;
        }
    };
    let src = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v & 0xffff,
        None => {
            log::trace!("pinsrw: error getting source operand");
            return false;
        }
    };
    let sel = match emu.get_operand_value(ins, 2, true) {
        Some(v) => (v & 0x7) as u32,
        None => {
            log::trace!("pinsrw: error getting selector");
            return false;
        }
    };

    let shift = sel * 16;
    dest = (dest & !(0xffffu128 << shift)) | ((src as u128) << shift);
    emu.set_operand_xmm_value_128(ins, 0, dest);
    true
}
