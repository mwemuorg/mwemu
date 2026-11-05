use crate::emu;

pub fn GetConsoleHistoryInfo(emu: &mut emu::Emu) {
    let info_ptr = emu.regs().rcx;
    log_red!(emu, "kernel32!GetConsoleHistoryInfo ptr:0x{:x}", info_ptr);
    if info_ptr != 0 {
        emu.maps.write_dword(info_ptr, 16);
        emu.maps.write_dword(info_ptr + 4, 50);
        emu.maps.write_dword(info_ptr + 8, 4);
        emu.maps.write_dword(info_ptr + 12, 0);
    }
    emu.regs_mut().rax = 1;
}
