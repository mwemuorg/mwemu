use crate::emu;

pub fn SetRectEmpty(emu: &mut emu::Emu) {
    let rect = emu.regs().rcx;
    if rect != 0 {
        emu.maps.write_dword(rect, 0);
        emu.maps.write_dword(rect + 4, 0);
        emu.maps.write_dword(rect + 8, 0);
        emu.maps.write_dword(rect + 12, 0);
    }
    log_red!(emu, "user32!SetRectEmpty 0x{:x}", rect);
    emu.regs_mut().rax = 1;
}
