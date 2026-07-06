use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let st0: f64 = emu.fpu_mut().get_st(0);

    if st0.is_nan() {
        // Undefined (QNaN)
        emu.fpu_mut().set_status_c0(true);
        emu.fpu_mut().set_status_c1(true);
        emu.fpu_mut().set_status_c2(true);
    } else if st0 == 0.0 {
        // Zero or negative zero
        emu.fpu_mut().set_status_c1(true);
        emu.fpu_mut().set_status_c2(false);
        emu.fpu_mut().set_status_c0(st0.is_sign_negative());
    } else if st0.is_infinite() {
        // Positive or negative Infinite
        emu.fpu_mut().set_status_c1(false);
        emu.fpu_mut().set_status_c2(true);
        emu.fpu_mut().set_status_c0(st0.is_sign_negative());
    } else if st0.abs() < std::f64::MIN_POSITIVE {
        // Denormal
        emu.fpu_mut().set_status_c0(true);
        emu.fpu_mut().set_status_c1(true);
        emu.fpu_mut().set_status_c2(false);
    } else {
        // Normal positive or negative
        emu.fpu_mut().set_status_c0(st0.is_sign_negative());
        emu.fpu_mut().set_status_c1(false);
        emu.fpu_mut().set_status_c2(false);
    }
    true
}
