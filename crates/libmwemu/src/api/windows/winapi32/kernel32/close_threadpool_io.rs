use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CloseThreadpoolIo(emu: &mut emu::Emu) {
    let pio = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!CloseThreadpoolIo pio:0x{:x}", pio);
    helper::handler_close(pio);
    stdcall_ret(emu, 1);
}
