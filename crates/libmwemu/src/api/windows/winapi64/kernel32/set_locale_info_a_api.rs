use crate::emu;

pub fn SetLocaleInfoA_api(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "kernel32!{} => TRUE", api);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
