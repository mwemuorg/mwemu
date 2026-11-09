use crate::emu;

pub(super) fn set_invalid_parameter_handler(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!_set_invalid_parameter_handler");
    emu.stack_pop32(false);
    emu.regs_mut().rax = 0;
}
