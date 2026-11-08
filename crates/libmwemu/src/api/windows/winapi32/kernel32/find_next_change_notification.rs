use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FindNextChangeNotification(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!FindNextChangeNotification h:0x{:x}", handle);
    if helper::handler_exist(handle) {
        emu.set_last_error(0);
        emu.regs_mut().rax = 1; // TRUE
    } else {
        emu.regs_mut().rax = 0; // FALSE
    }
    stdcall_ret(emu, 1);
}
