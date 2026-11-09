use crate::emu;

pub fn GetMonitorInfoA(emu: &mut emu::Emu) {
    let hmonitor = emu.regs().rcx;
    let info_ptr = emu.regs().rdx;
    if info_ptr != 0 {
        emu.maps.write_dword(info_ptr + 4, 0);
        emu.maps.write_dword(info_ptr + 8, 0);
        emu.maps.write_dword(info_ptr + 12, 1920);
        emu.maps.write_dword(info_ptr + 16, 1080);
        emu.maps.write_dword(info_ptr + 20, 0);
        emu.maps.write_dword(info_ptr + 24, 0);
        emu.maps.write_dword(info_ptr + 28, 1920);
        emu.maps.write_dword(info_ptr + 32, 1080);
        emu.maps.write_dword(info_ptr + 36, 1);
    }
    log_red!(
        emu,
        "user32!GetMonitorInfoA hmon:0x{:x} info:0x{:x}",
        hmonitor,
        info_ptr
    );
    emu.regs_mut().rax = 1;
}
