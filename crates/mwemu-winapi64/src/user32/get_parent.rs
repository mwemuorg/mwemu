use crate::emu;
use crate::winapi::helper;

pub fn GetParent(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "user32!GetParent hwnd:0x{:x}", hwnd);
    let parent = if helper::handler_exist(hwnd) {
        helper::handler_create("parent_hwnd")
    } else {
        0
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = parent;
}
