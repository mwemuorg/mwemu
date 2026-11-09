use crate::emu;
use crate::winapi::helper;

pub fn ReleaseDC(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let hdc = emu.regs().rdx;
    log_red!(emu, "gdi32!ReleaseDC hwnd:0x{:x} hdc:0x{:x}", hwnd, hdc);
    if helper::handler_exist(hdc) {
        helper::handler_close(hdc);
    }
    emu.regs_mut().rax = 1;
}
