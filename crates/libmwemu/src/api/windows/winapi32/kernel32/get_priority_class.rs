use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetPriorityClass(emu: &mut emu::Emu) {
    let p0 = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!GetPriorityClass");
    let _ = helper::handler_exist(arg(emu, 0) as u64);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x20;
    stdcall_ret(emu, 1);
}
