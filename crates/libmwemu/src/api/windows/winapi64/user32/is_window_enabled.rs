use crate::emu;
use crate::winapi::helper;

pub fn IsWindowEnabled(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "user32!IsWindowEnabled hwnd:0x{:x}", hwnd);
    let handle = helper::handler_create("IsWindowEnabled");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
