use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::utils::bits::low_bits_u64;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let src = match emu.get_operand_value(ins, 2, true) {
        Some(v) => v,
        None => {
            log::trace!("mulx: error reading src operand");
            return false;
        }
    };

    let bits = emu.get_operand_sz(ins, 0);
    let factor = low_bits_u64(emu.regs().rdx, bits) as u128;
    let product = (low_bits_u64(src, bits) as u128) * factor;
    // Write the low half first so the high half wins when both registers alias.
    emu.set_operand_value(ins, 1, low_bits_u64(product as u64, bits))
        && emu.set_operand_value(ins, 0, (product >> bits) as u64)
}
