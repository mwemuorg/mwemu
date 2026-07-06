use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let st0 = emu.fpu_mut().get_st(0);
    emu.fpu_mut().set_status_c0(st0 < 0.0);
    emu.fpu_mut().set_status_c2(st0.is_nan());
    emu.fpu_mut().set_status_c3(st0 == 0.0);
    true
}
