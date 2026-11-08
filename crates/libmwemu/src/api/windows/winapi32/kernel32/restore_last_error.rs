use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn RestoreLastError(emu: &mut emu::Emu) {
    let error = arg(emu, 0) as u64 as u32;

    log_red!(emu, "kernel32!RestoreLastError error:0x{:x}", error);

    emu.set_last_error(error as u64);
    stdcall_ret(emu, 1);
}
