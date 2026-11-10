use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let res = emu.fpu_mut().get_st(0) as u64;

    if !emu.set_operand_value(ins, 0, res) {
        return false;
    }
    emu.sync_fpu_ip();
    true
}
