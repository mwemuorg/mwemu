use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ClosePackageInfo(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!ClosePackageInfo h:0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 0; // ERROR_SUCCESS
    stdcall_ret(emu, 1);
}
