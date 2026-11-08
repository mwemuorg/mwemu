use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CloseThreadpoolTimer(emu: &mut emu::Emu) {
    let timer = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!CloseThreadpoolTimer timer:0x{:x}", timer);
    helper::handler_close(timer);
    helper::advance_tick(emu, 1);
    stdcall_ret(emu, 1);
}
