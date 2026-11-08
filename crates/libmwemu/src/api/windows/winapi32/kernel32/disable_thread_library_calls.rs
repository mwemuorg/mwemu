use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn DisableThreadLibraryCalls(emu: &mut emu::Emu) {
    let p0 = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!DisableThreadLibraryCalls");
    let h = arg(emu, 0) as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
