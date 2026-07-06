use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let value0 = emu.get_operand_value(ins, 0, false).unwrap_or(0) as usize;
    let value2 = emu.get_operand_value(ins, 1, false).unwrap_or(2) as usize;

    let sti = emu.fpu_mut().get_st(value0);
    let stj = emu.fpu_mut().get_st(value2);

    emu.fpu_mut().set_status_c0(sti < stj);
    emu.fpu_mut().set_status_c2(sti.is_nan() || stj.is_nan());
    emu.fpu_mut().set_status_c3(sti == stj);

    emu.fpu_mut().pop_f64();
    true
}
