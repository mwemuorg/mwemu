use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::utils::bits::low_bits_u128;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let source = match emu.get_operand_xmm_value_128(ins, 1, true) {
        Some(v) => v,
        None => {
            log::trace!("pextrw: error getting source xmm");
            return false;
        }
    };
    let lanes = emu.get_operand_sz(ins, 1) / u16::BITS;
    let sel = match emu.get_operand_value(ins, 2, true) {
        Some(v) => v & (lanes - 1) as u64,
        None => {
            log::trace!("pextrw: error getting selector");
            return false;
        }
    };

    let word = low_bits_u128(source >> (sel * u16::BITS as u64), u16::BITS) as u64;
    emu.set_operand_value(ins, 0, word)
}
