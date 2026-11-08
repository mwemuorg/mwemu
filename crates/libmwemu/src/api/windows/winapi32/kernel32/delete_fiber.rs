use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn DeleteFiber(emu: &mut emu::Emu) {
    let fiber = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!DeleteFiber fiber:0x{:x}", fiber);
    helper::handler_close(fiber);
    stdcall_ret(emu, 1);
}
