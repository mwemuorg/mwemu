use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ResetEvent(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!ResetEvent handle:0x{:x}", handle);
    if helper::handler_exist(handle) {
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(6);
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 1);
}
