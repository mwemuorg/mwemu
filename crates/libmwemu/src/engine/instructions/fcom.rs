use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let st0 = emu.fpu_mut().get_st(0);

    let value1 = emu.get_operand_value(ins, 1, false).unwrap_or_default();

    let st4 = emu.fpu_mut().get_st(value1 as usize);

    if st0.is_nan() || st4.is_nan() {
        emu.fpu_mut().set_status_c0(false);
        emu.fpu_mut().set_status_c2(true);
        emu.fpu_mut().set_status_c3(false);
    } else {
        emu.fpu_mut().set_status_c0(st0 < st4);
        emu.fpu_mut().set_status_c2(false);
        emu.fpu_mut().set_status_c3(st0 == st4);
    }
    emu.sync_fpu_ip();
    true
}
