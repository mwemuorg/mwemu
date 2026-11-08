use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn IsUserCetAvailableInEnvironment(emu: &mut emu::Emu) {
    let user_cet_env = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!IsUserCetAvailableInEnvironment env:0x{:x} => FALSE",
        user_cet_env
    );
    emu.set_last_error(0);
    let h = arg(emu, 0) as u64;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 1);
}
