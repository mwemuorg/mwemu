use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetConsoleMode(emu: &mut emu::Emu) {
    let handle = arg(emu, 0);
    let mode_ptr = arg(emu, 1) as u64;
    log_red!(emu, "kernel32!GetConsoleMode hndl: 0x{:x}", handle);
    emu.maps.write_dword(mode_ptr, 3); // ENABLE_PROCESSED_INPUT | ENABLE_LINE_INPUT
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
