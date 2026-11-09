use crate::emu;
use crate::winapi::helper;

pub fn SetActiveWindow(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "user32!SetActiveWindow hwnd:0x{:x}", hwnd);
    if hwnd != 0 {
        let _ = helper::handler_create("ActiveWindow");
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = hwnd;
}
