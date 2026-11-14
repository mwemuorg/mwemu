use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn ScrollWindowEx(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let dx = emu.regs().rdx as i32;
    let dy = emu.regs().r8 as i32;
    let _lp_scroll = emu.regs().r9;
    let _lp_clip = WinApiAbi::arg(emu, 4);
    let hrgn_update = WinApiAbi::arg(emu, 5);
    let lp_rc_update = WinApiAbi::arg(emu, 6);
    let _flags = WinApiAbi::arg(emu, 7);
    if lp_rc_update != 0 {
        emu.maps.write_dword(lp_rc_update, 0);
        emu.maps.write_dword(lp_rc_update + 4, 0);
        emu.maps.write_dword(lp_rc_update + 8, 0);
        emu.maps.write_dword(lp_rc_update + 12, 0);
    }
    log_red!(
        emu,
        "user32!ScrollWindowEx hwnd:0x{:x} dx:{} dy:{} hrgn:0x{:x}",
        hwnd,
        dx,
        dy,
        hrgn_update
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // SIMPLEREGION
}
