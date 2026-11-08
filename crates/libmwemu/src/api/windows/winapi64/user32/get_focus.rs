use crate::emu;
use crate::winapi::helper;

pub(super) fn GetFocus(emu: &mut emu::Emu) {
    let hwnd = helper::handler_create("hwnd://focus");
    log_red!(emu, "user32!GetFocus => 0x{:x}", hwnd);
    emu.regs_mut().rax = hwnd;
}
