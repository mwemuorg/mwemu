use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn IsValidNLSVersion(emu: &mut emu::Emu) {
    let func = arg(emu, 0) as u64;
    let lp_locale_name = arg(emu, 1) as u64;
    let lp_version_info = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!IsValidNLSVersion func:{} locale:0x{:x} ver:0x{:x}",
        func,
        lp_locale_name,
        lp_version_info
    );
    emu.set_last_error(0);
    let h = arg(emu, 0) as u64;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 3);
}
