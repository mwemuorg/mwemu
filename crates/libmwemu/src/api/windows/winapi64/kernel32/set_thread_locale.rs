use crate::emu;

pub fn SetThreadLocale(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;

    log_red!(emu, "kernel32!SetThreadLocale locale:0x{:x}", locale);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
