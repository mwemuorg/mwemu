use crate::emu;
use crate::winapi::helper;

pub(super) fn GetDC(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "user32!GetDC hwnd=0x{:x}", hwnd);
    emu.regs_mut().rax = helper::handler_create("hdc://");
}
