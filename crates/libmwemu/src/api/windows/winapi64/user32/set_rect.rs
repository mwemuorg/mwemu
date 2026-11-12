use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SetRect(emu: &mut emu::Emu) {
    let rect = emu.regs().rcx;
    let left = emu.regs().rdx as u32;
    let top = emu.regs().r8 as u32;
    let right = emu.regs().r9 as u32;
    let bottom = WinApiAbi::arg(emu, 4) as u32;
    if rect != 0 {
        emu.maps.write_dword(rect, left);
        emu.maps.write_dword(rect + 4, top);
        emu.maps.write_dword(rect + 8, right);
        emu.maps.write_dword(rect + 12, bottom);
    }
    log_red!(
        emu,
        "user32!SetRect ({},{},{},{}) => 0x{:x}",
        left,
        top,
        right,
        bottom,
        rect
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
