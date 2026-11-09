use crate::emu;

pub(super) fn AdjustWindowRect(emu: &mut emu::Emu) {
    let lprect = emu.regs().rcx;
    let style = emu.regs().rdx;
    let bmenu = emu.regs().r8;
    log_red!(
        emu,
        "user32!AdjustWindowRect rect:0x{:x} style:0x{:x} menu:{}",
        lprect,
        style,
        bmenu
    );
    if lprect != 0 && emu.maps.is_mapped(lprect) {
        let left = emu.maps.read_dword(lprect).unwrap_or(0) as i32 - 8;
        let top = emu.maps.read_dword(lprect + 4).unwrap_or(0) as i32 - 30;
        let right = emu.maps.read_dword(lprect + 8).unwrap_or(0) as i32 + 8;
        let bottom = emu.maps.read_dword(lprect + 12).unwrap_or(0) as i32 + 8;
        emu.maps.write_dword(lprect, left as u32);
        emu.maps.write_dword(lprect + 4, top as u32);
        emu.maps.write_dword(lprect + 8, right as u32);
        emu.maps.write_dword(lprect + 12, bottom as u32);
    }
    emu.regs_mut().rax = 1;
}
