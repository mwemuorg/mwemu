use crate::emu;

pub fn GetThreadLocale(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetThreadLocale");
    emu.set_last_error(0);
    //emu.regs_mut().rax = constants::LOCALE_USER_DEFAULT; // TODO: 0x400 LOCALE_USER_DEFAULT or 0x409?
    emu.regs_mut().rax = 0x409; // English (United States)
}
