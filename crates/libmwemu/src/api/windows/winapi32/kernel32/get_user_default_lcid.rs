use crate::emu;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::nls::*;

pub fn GetUserDefaultLCID(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetUserDefaultLCID");
    emu.regs_mut().rax = LOCALE_USER_DEFAULT;
    stdcall_ret(emu, 0);
}
