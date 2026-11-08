use crate::emu;
use crate::winapi::helper;

pub(super) fn ReleaseDC(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let hdc = emu.regs().rdx;
    log_red!(emu, "user32!ReleaseDC hwnd=0x{:x} hdc=0x{:x}", hwnd, hdc);
    helper::handler_close(hdc);
    emu.regs_mut().rax = 1;
}
