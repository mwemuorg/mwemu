use crate::emu;
use crate::winapi::helper;

pub(super) fn SetForegroundWindow(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "user32!SetForegroundWindow hwnd:0x{:x}", hwnd);
    if helper::handler_exist(hwnd) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
