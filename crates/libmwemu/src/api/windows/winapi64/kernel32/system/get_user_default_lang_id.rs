use crate::emu::Emu;
use crate::windows::constants::nls::*;

pub fn GetUserDefaultLangID(emu: &mut Emu) {
    log_red!(emu, "kernel32!GetUserDefaultLangID");
    emu.regs_mut().rax = EN_US_LOCALE as u64;
}
