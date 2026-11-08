use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn DeleteUmsThreadContext(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    helper::handler_close(handle);
    log_red!(emu, "kernel32!DeleteUmsThreadContext handle:0x{:x}", handle);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
