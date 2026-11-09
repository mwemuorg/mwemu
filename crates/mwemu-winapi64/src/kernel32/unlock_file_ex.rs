use crate::emu;
use crate::winapi::helper;

pub fn UnlockFileEx(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let dw_reserved = emu.regs().rdx;
    let n_bytes_to_unlock_low = emu.regs().r8;
    let n_bytes_to_unlock_high = emu.regs().r9;

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
}
