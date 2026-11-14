use crate::emu;

pub fn GetClipBox(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let rect = emu.regs().rdx;
    if rect != 0 {
        emu.maps.write_dword(rect, 0);
        emu.maps.write_dword(rect + 4, 0);
        emu.maps.write_dword(rect + 8, 1920);
        emu.maps.write_dword(rect + 12, 1080);
    }
    log_red!(emu, "gdi32!GetClipBox hdc:0x{:x} rect:0x{:x}", hdc, rect);
    emu.regs_mut().rax = 1; // SIMPLEREGION
}
