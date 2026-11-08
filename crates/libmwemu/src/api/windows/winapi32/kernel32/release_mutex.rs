use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ReleaseMutex(emu: &mut emu::Emu) {
    let h_mutex = arg(emu, 0) as u64;

    log_red!(emu, "kernel32!ReleaseMutex hMutex:0x{:x}", h_mutex);

    if !helper::handler_exist(h_mutex) {
        log_red!(emu, "kernel32!ReleaseMutex invalid handle");
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
    stdcall_ret(emu, 1);
}
