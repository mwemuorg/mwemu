use crate::emu;
use crate::windows::constants;

pub fn GetUserDefaultUILanguage(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetUserDefaultUILanguage (0x0409 en_US)");
    emu.set_last_error(0);
    emu.regs_mut().rax = constants::EN_US_LOCALE as u64;
}
