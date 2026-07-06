use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Red"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let array_index = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => {
            log::trace!("cannot read first opreand of bound");
            return false;
        }
    };
    let lower_upper_bound = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => {
            log::trace!("cannot read second opreand of bound");
            return false;
        }
    };

    log::trace!(
        "bound idx:{} lower_upper:{}",
        array_index,
        lower_upper_bound
    );
    log::trace!("Bound unimplemented");
    false
    // https://www.felixcloutier.com/x86/bound
    //true
}
