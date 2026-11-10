use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::utils::bits::low_bits_u64;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let mut dest = match emu.get_operand_xmm_value_128(ins, 0, true) {
        Some(v) => v,
        None => {
            log::trace!("pinsrw: error getting destination xmm");
            return false;
        }
    };
    let src = match emu.get_operand_value(ins, 1, true) {
        Some(v) => low_bits_u64(v, u16::BITS),
        None => {
            log::trace!("pinsrw: error getting source operand");
            return false;
        }
    };
    let lanes = emu.get_operand_sz(ins, 0) / u16::BITS;
    let sel = match emu.get_operand_value(ins, 2, true) {
        Some(v) => v as u32 & (lanes - 1),
        None => {
            log::trace!("pinsrw: error getting selector");
            return false;
        }
    };

    let shift = sel * u16::BITS;
    dest = (dest & !((u16::MAX as u128) << shift)) | ((src as u128) << shift);
    emu.set_operand_xmm_value_128(ins, 0, dest)
}
