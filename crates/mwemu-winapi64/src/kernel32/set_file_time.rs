use crate::emu;
use crate::winapi::helper;

pub fn SetFileTime(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let lp_creation_time = emu.regs().rdx;
    let lp_last_access_time = emu.regs().r8;
    let lp_last_write_time = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!SetFileTime hFile:0x{:x} creation:0x{:x} access:0x{:x} write:0x{:x}",
        h_file,
        lp_creation_time,
        lp_last_access_time,
        lp_last_write_time
    );

    if !helper::handler_exist(h_file) {
        log_red!(emu, "kernel32!SetFileTime invalid handle");
        emu.set_last_error(6); // ERROR_INVALID_HANDLE
        emu.regs_mut().rax = 0;
        return;
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
