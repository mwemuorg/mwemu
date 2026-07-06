use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let st0 = emu.fpu_mut().get_st(0); // ST(0) = divisor
    let st1 = emu.fpu_mut().get_st(1); // ST(1) = dividend

    let result = st1 / st0; // ST(1) = ST(1) / ST(0)
    emu.fpu_mut().set_st(1, result);

    emu.fpu_mut().pop_f64(); // Remove ST(0)
    emu.sync_fpu_ip();
    true
}
