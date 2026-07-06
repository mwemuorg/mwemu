use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    if ins.op_count() == 2 {
        let i = emu.fpu().reg_to_id(ins.op_register(0));
        let j = emu.fpu().reg_to_id(ins.op_register(1));
        emu.fpu_mut().add(i, j);
        emu.fpu_mut().pop_f64();
    } else {
        unreachable!();
    }
    emu.sync_fpu_ip();
    true
}
