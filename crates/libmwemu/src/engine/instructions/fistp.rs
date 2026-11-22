use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::fpu::FPU_INVALID_OPERATION;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), &DecodedInstruction::X86(*ins));

    let width = emu.get_operand_sz(ins, 0);
    if !matches!(width, 16 | 32 | 64) {
        return false;
    }
    let value = emu.fpu_mut().get_st_f80_copy(0);
    let integer = value.signed_integer(width, emu.fpu().get_ctrl());
    let bits = integer.map_or(1u64 << (width - 1), |n| n as u64);
    if !emu.set_operand_value(ins, 0, bits) {
        return false;
    }
    if integer.is_none() {
        emu.fpu_mut().status |= FPU_INVALID_OPERATION;
    }
    emu.fpu_mut().pop_f64();
    emu.sync_fpu_ip();
    true
}
