use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CloseThreadpool(emu: &mut emu::Emu) {
    let pool = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!CloseThreadpool pool:0x{:x}", pool);
    helper::handler_close(pool);
    stdcall_ret(emu, 1);
}
