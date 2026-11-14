use crate::emu;
use crate::winapi::helper;

pub fn GetDC(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "gdi32!GetDC hwnd:0x{:x}", hwnd);
    emu.regs_mut().rax = helper::handler_create("gdi32://dc");
}
