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

    if st0.is_nan() || st1.is_nan() {
        emu.fpu_mut().set_status_c0(true);
        emu.fpu_mut().set_status_c2(true);
        emu.fpu_mut().set_status_c3(true);
    } else if st0 == st1 {
        emu.fpu_mut().set_status_c0(false);
        emu.fpu_mut().set_status_c2(false);
        emu.fpu_mut().set_status_c3(true);
    } else if st0 > st1 {
        emu.fpu_mut().set_status_c0(true);
        emu.fpu_mut().set_status_c2(false);
        emu.fpu_mut().set_status_c3(false);
    } else {
        // st0 < st1
        emu.fpu_mut().set_status_c0(false);
        emu.fpu_mut().set_status_c2(false);
        emu.fpu_mut().set_status_c3(false);
    }

    emu.sync_fpu_ip();
    true
}
