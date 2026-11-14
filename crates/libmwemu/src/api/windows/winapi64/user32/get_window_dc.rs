use crate::emu;
use crate::winapi::helper;

pub fn GetWindowDC(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let hdc = helper::handler_create("dc");
    log_red!(emu, "user32!GetWindowDC hwnd:0x{:x} => 0x{:x}", hwnd, hdc);
    emu.set_last_error(0);
    emu.regs_mut().rax = hdc;
}
