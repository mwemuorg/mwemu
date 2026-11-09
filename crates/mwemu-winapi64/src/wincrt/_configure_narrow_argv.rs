use crate::emu;

pub(super) fn _configure_narrow_argv(emu: &mut emu::Emu) {
    let mode = emu.regs().rcx;
    log_red!(emu, "wincrt!_configure_narrow_argv mode: 0x{:x}", mode);
    emu.regs_mut().rax = 0;
}
