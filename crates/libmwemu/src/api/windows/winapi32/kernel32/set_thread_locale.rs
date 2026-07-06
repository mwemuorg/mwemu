use crate::emu;

pub fn SetThreadLocale(emu: &mut emu::Emu) {
    let locale = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!SetThreadLocale cannot read locale param");

    log_red!(emu, "kernel32!SetThreadLocale {}", locale);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
