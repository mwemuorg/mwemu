use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FindCloseChangeNotification(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!FindCloseChangeNotification h:0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 1; // TRUE
    stdcall_ret(emu, 1);
}
