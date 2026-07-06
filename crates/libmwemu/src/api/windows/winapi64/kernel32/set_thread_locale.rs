use crate::emu;

pub fn SetThreadLocale(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!SetThreadLocale");
    // TODO: do something
    emu.regs_mut().rax = 1;
}
