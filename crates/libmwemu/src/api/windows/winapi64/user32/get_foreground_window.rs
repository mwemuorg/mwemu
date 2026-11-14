use crate::emu;
use crate::winapi::helper;

pub(super) fn GetForegroundWindow(emu: &mut emu::Emu) {
    let hwnd = helper::handler_create("hwnd://foreground");
    log_red!(emu, "user32!GetForegroundWindow => 0x{:x}", hwnd);
    emu.regs_mut().rax = hwnd;
}
