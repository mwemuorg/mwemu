use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    const RPL_MASK: u64 = 0b11;
    let dest = try_operand!(emu.get_operand_value(ins, 0, true));
    let source = try_operand!(emu.get_operand_value(ins, 1, true));
    let adjusted = (dest & RPL_MASK) < (source & RPL_MASK);
    let result = if adjusted {
        (dest & !RPL_MASK) | (source & RPL_MASK)
    } else {
        dest
    };
    // The memory operand requires write access even when its RPL is unchanged.
    if !emu.set_operand_value(ins, 0, result) {
        return false;
    }
    emu.flags_mut().f_zf = adjusted;
    true
}
