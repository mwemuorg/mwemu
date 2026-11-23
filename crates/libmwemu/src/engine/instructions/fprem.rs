use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));
    let fpu = emu.fpu_mut();
    let mut dividend = fpu.get_st_f80_copy(0);
    let divisor = fpu.get_st_f80_copy(1);
    let quotient = dividend.remainder(divisor);
    fpu.set_st_u80(0, dividend.get());
    fpu.set_status_c2(quotient.is_none());
    if let Some(bits) = quotient {
        fpu.set_status_c0(bits & 4 != 0);
        fpu.set_status_c3(bits & 2 != 0);
        fpu.set_status_c1(bits & 1 != 0);
    }
    emu.sync_fpu_ip();
    true
}
