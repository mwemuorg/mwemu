use crate::color;
use crate::emu::Emu;
use crate::fpu::f80::F80;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );

    let st0_idx = 0; // src, always ST(0)
    let st1_idx = emu.get_operand_value(ins, 0, false).unwrap_or(1) as usize; // dest
    log::trace!("st0_idx: {}, st1_idx: {}", st0_idx, st1_idx);

    let st0 = emu.fpu_mut().get_st_f80_copy(st0_idx);
    let st1 = emu.fpu_mut().get_st_f80_copy(st1_idx);
    log::trace!("st0: {:x}, st1: {:x}", st0.get(), st1.get());

    if st0.is_empty() || st1.is_empty() {
        emu.fpu_mut().set_st(st1_idx, F80::QNaN().get_f64());
    } else {
        emu.fpu_mut().sub(st1_idx, st0_idx);
    }

    emu.fpu_mut().pop_f64(); // pops ST(0) properly
    emu.sync_fpu_ip();
    true
}
