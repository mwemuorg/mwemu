use crate::emu;

pub fn GlobalMemoryStatus(emu: &mut emu::Emu) {
    let lp_buffer = emu.regs().rcx;

    log_red!(emu, "kernel32!GlobalMemoryStatus buf:0x{:x}", lp_buffer);

    if lp_buffer != 0 {
        emu.maps.write_dword(lp_buffer, 32);
        emu.maps.write_dword(lp_buffer + 4, 50);
        emu.maps.write_dword(lp_buffer + 8, 0x2000_0000);
        emu.maps.write_dword(lp_buffer + 12, 0x1000_0000);
        emu.maps.write_dword(lp_buffer + 16, 0x8000_0000u32);
        emu.maps.write_dword(lp_buffer + 20, 0x4000_0000);
        emu.maps.write_dword(lp_buffer + 24, 0xFFFF_FFFF);
        emu.maps.write_dword(lp_buffer + 28, 0x8000_0000u32);
    }
}
