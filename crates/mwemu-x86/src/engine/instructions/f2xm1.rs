use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let st0 = emu.fpu_mut().get_st(0);
    let result = (2.0f64.powf(st0)) - 1.0;
    emu.fpu_mut().set_st(0, result);
    emu.sync_fpu_ip();
    true
}
