use crate::emu;

pub fn SetLocaleInfoA(emu: &mut emu::Emu) {
    set_locale_info(emu, "SetLocaleInfoA");
}

pub fn SetLocaleInfoW(emu: &mut emu::Emu) {
    set_locale_info(emu, "SetLocaleInfoW");
}

fn set_locale_info(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "kernel32!{} => TRUE", api);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
