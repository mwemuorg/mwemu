use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let value = match emu.get_operand_value(ins, 0, false) {
        Some(v) => v as u16,
        None => return false,
    };

    emu.fpu_mut().set_ctrl(value);
    emu.sync_fpu_ip();
    true
}
