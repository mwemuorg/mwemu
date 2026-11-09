use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let st0 = emu.fpu_mut().get_st(0);
    let st1 = emu.fpu_mut().get_st(1);
    let result = (st1 / st0).atan();
    emu.fpu_mut().set_st(1, result);
    emu.fpu_mut().pop_f64();
    emu.sync_fpu_ip();
    true
}
