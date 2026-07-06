use crate::emu;
use crate::windows::constants;

pub fn GetThreadUILanguage(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetThreadUILanguage (0x0409 en_US)");

    emu.regs_mut().rax = constants::EN_US_LOCALE as u64;
}
