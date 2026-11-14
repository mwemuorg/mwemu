use crate::emu;
use crate::winapi::helper;

pub fn GetActiveWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetActiveWindow");
    emu.set_last_error(0);
    emu.regs_mut().rax = helper::handler_create("hwnd://active");
}
