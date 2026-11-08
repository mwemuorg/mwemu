use crate::emu;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants;

pub fn GetUserDefaultLCID(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetUserDefaultLCID");
    emu.regs_mut().rax = constants::LOCALE_USER_DEFAULT;
    stdcall_ret(emu, 0);
}
