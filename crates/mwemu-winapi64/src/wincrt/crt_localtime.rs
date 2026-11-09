use super::allocate_memory;
use crate::emu;

pub(super) fn crt_localtime(emu: &mut emu::Emu) {
    let tm_addr = allocate_memory(emu, 64).unwrap_or(0);
    if tm_addr != 0 {
        emu.maps.write_dword(tm_addr, 0); // tm_sec
        emu.maps.write_dword(tm_addr + 4, 0); // tm_min
        emu.maps.write_dword(tm_addr + 8, 12); // tm_hour
        emu.maps.write_dword(tm_addr + 12, 1); // tm_mday
        emu.maps.write_dword(tm_addr + 16, 0); // tm_mon
        emu.maps.write_dword(tm_addr + 20, 124); // tm_year (2024-1900)
        emu.maps.write_dword(tm_addr + 24, 1); // tm_wday
        emu.maps.write_dword(tm_addr + 28, 0); // tm_yday
        emu.maps.write_dword(tm_addr + 32, 0); // tm_isdst
    }
    log_red!(emu, "wincrt!localtime => 0x{:x}", tm_addr);
    emu.regs_mut().rax = tm_addr;
}
