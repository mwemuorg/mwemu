use crate::emu;
use crate::winapi::helper;

pub fn get_system_default_ui_language(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetSystemDefaultUILanguage");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x0409; // en-US
}
