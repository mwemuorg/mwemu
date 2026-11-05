use crate::{emu, windows::constants};

pub fn GetUserDefaultLCID(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetUserDefaultLCID");
    emu.set_last_error(0);
    emu.regs_mut().rax = constants::LOCALE_USER_DEFAULT;
}
