use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ClosePseudoConsole(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!ClosePseudoConsole h:0x{:x}", handle);
    helper::handler_close(handle);
    stdcall_ret(emu, 1);
}
