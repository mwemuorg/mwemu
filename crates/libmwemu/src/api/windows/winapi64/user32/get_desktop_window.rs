use crate::emu;
use crate::winapi::helper;

pub(super) fn GetDesktopWindow(emu: &mut emu::Emu) {
    let h = helper::handler_create("desktop_window");
    log_red!(emu, "user32!GetDesktopWindow => 0x{:x}", h);
    emu.regs_mut().rax = h;
}
