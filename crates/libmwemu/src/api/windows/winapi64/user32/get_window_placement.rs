use crate::emu;

pub fn GetWindowPlacement(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let lpwndpl = emu.regs().rdx;
    if lpwndpl != 0 {
        emu.maps.write_dword(lpwndpl, 44); // length
        emu.maps.write_dword(lpwndpl + 4, 0); // flags
        emu.maps.write_dword(lpwndpl + 8, 1); // showCmd = SW_SHOWNORMAL
        emu.maps.write_dword(lpwndpl + 12, 0); // ptMinPosition.x
        emu.maps.write_dword(lpwndpl + 16, 0); // ptMinPosition.y
        emu.maps.write_dword(lpwndpl + 20, 0); // ptMaxPosition.x
        emu.maps.write_dword(lpwndpl + 24, 0); // ptMaxPosition.y
        emu.maps.write_dword(lpwndpl + 28, 0); // rcNormalPosition.left
        emu.maps.write_dword(lpwndpl + 32, 0); // rcNormalPosition.top
        emu.maps.write_dword(lpwndpl + 36, 800); // rcNormalPosition.right
        emu.maps.write_dword(lpwndpl + 40, 600); // rcNormalPosition.bottom
    }
    log_red!(
        emu,
        "user32!GetWindowPlacement hwnd:0x{:x} lpwndpl:0x{:x}",
        hwnd,
        lpwndpl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
