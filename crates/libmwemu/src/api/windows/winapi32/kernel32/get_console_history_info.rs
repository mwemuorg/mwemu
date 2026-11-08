use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetConsoleHistoryInfo(emu: &mut emu::Emu) {
    let info_ptr = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!GetConsoleHistoryInfo ptr:0x{:x}", info_ptr);
    if info_ptr != 0 {
        emu.maps.write_dword(info_ptr, 16);
        emu.maps.write_dword(info_ptr + 4, 50);
        emu.maps.write_dword(info_ptr + 8, 4);
        emu.maps.write_dword(info_ptr + 12, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
