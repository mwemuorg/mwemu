use crate::emu;
use crate::winapi::helper;

pub fn GetMenu(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "user32!GetMenu hwnd:0x{:x}", hwnd);
    let hmenu = if helper::handler_exist(hwnd) {
        helper::handler_create("menu")
    } else {
        0
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = hmenu;
}
