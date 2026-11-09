use crate::emu;

pub(super) fn _initialize_narrow_environment(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!_initialize_narrow_environment");
    emu.regs_mut().rax = 0;
}
