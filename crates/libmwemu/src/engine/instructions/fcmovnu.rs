use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    if !emu.flag_pf() {
        emu.fpu_mut().move_reg_to_st0(ins.op_register(1));
    }

    emu.sync_fpu_ip();
    true
}
