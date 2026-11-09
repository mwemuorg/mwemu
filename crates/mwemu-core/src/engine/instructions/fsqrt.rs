use crate::color;
use crate::emu::Emu;
use crate::fpu::f80::F80;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Green"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    let st0 = emu.fpu_mut().get_st(0);

    if st0 < 0.0 {
        if emu.cfg.verbose >= 1 {
            log::trace!("/!\\ sqrt of negative float");
        }
        emu.fpu_mut().set_st_u80(0, F80::QNaN().get());
    } else {
        emu.fpu_mut().set_st(0, st0.sqrt());
    }
    emu.sync_fpu_ip();
    true
}
