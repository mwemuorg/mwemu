use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CloseThreadpoolWait(emu: &mut emu::Emu) {
    let pwa = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!CloseThreadpoolWait pwa:0x{:x}", pwa);
    helper::handler_close(pwa);
    helper::advance_tick(emu, 1);
    stdcall_ret(emu, 1);
}
