use crate::emu;
use crate::winapi::helper;

pub(super) fn BeginPaint(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let ps_ptr = emu.regs().rdx;
    let hdc = helper::handler_create("hdc://paint");
    if ps_ptr != 0 {
        // PAINTSTRUCT: hdc(8) + fErase(4) + rcPaint(16) + ...
        emu.maps.write_qword(ps_ptr, hdc);
        emu.maps.write_dword(ps_ptr + 8, 0); // fErase = FALSE
        // rcPaint: left, top, right, bottom
        emu.maps.write_dword(ps_ptr + 12, 0);
        emu.maps.write_dword(ps_ptr + 16, 0);
        emu.maps.write_dword(ps_ptr + 20, 1920);
        emu.maps.write_dword(ps_ptr + 24, 1080);
    }
    log_red!(
        emu,
        "user32!BeginPaint hwnd:0x{:x} => hdc:0x{:x}",
        hwnd,
        hdc
    );
    emu.regs_mut().rax = hdc;
}
