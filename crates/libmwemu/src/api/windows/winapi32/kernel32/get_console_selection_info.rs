use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetConsoleSelectionInfo(emu: &mut emu::Emu) {
    let info_ptr = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!GetConsoleSelectionInfo ptr:0x{:x}", info_ptr);
    if info_ptr != 0 {
        for i in 0..4u64 {
            emu.maps.write_dword(info_ptr + i * 4, 0);
        }
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
