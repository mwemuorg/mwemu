use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn LZCloseFile(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!LZCloseFile hFile:0x{:x}", h_file);
    let h = arg(emu, 0) as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    stdcall_ret(emu, 1);
}
