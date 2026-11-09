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

    let scale_factor = 2.0f64.powf(st1.trunc());
    let result = st0 * scale_factor;

    emu.fpu_mut().set_st(0, result);
    true
}
