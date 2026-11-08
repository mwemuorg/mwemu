use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetConsoleDisplayMode(emu: &mut emu::Emu) {
    let mode_ptr = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!GetConsoleDisplayMode ptr:0x{:x}", mode_ptr);
    if mode_ptr != 0 {
        emu.maps.write_dword(mode_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
