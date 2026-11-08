use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GlobalMemoryStatusEx(emu: &mut emu::Emu) {
    let lp_buffer = arg(emu, 0) as u64;

    log_red!(emu, "kernel32!GlobalMemoryStatusEx buf:0x{:x}", lp_buffer);

    if lp_buffer != 0 {
        emu.maps.write_dword(lp_buffer, 64);
        emu.maps.write_dword(lp_buffer + 4, 50);
        emu.maps.write_qword(lp_buffer + 8, 0x2_0000_0000);
        emu.maps.write_qword(lp_buffer + 16, 0x1_0000_0000);
        emu.maps.write_qword(lp_buffer + 24, 0x4_0000_0000);
        emu.maps.write_qword(lp_buffer + 32, 0x2_0000_0000);
        emu.maps.write_qword(lp_buffer + 40, 0x2_0000_0000);
        emu.maps.write_qword(lp_buffer + 48, 0x1_0000_0000);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
