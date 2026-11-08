use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn UnlockFileEx(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    let dw_reserved = arg(emu, 1) as u64;
    let n_bytes_to_unlock_low = arg(emu, 2) as u64;
    let n_bytes_to_unlock_high = arg(emu, 3) as u64;

    log_red!(
        emu,
        "kernel32!UnlockFileEx hFile:0x{:x} reserved:{} low:{} high:{}",
        h_file,
        dw_reserved,
        n_bytes_to_unlock_low,
        n_bytes_to_unlock_high
    );

    if !helper::handler_exist(h_file) {
        log_red!(emu, "kernel32!UnlockFileEx invalid handle");
        emu.set_last_error(6); // ERROR_INVALID_HANDLE
        emu.regs_mut().rax = 0;
        return;
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
    stdcall_ret(emu, 4);
}
