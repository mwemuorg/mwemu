use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetThreadPriority(emu: &mut emu::Emu) {
    let p0 = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!GetThreadPriority");
    let _ = helper::handler_exist(arg(emu, 0) as u64);
    emu.set_last_error(0);
    emu.regs_mut().rax = 2;
    stdcall_ret(emu, 1);
}
