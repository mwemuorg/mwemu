use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CloseThreadpoolWork(emu: &mut emu::Emu) {
    let work = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!CloseThreadpoolWork work:0x{:x}", work);
    helper::handler_close(work);
    stdcall_ret(emu, 1);
}
