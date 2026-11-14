use crate::emu;
use crate::winapi::helper;

pub fn SetCapture(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "user32!SetCapture hwnd:0x{:x}", hwnd);
    if hwnd != 0 {
        let _ = helper::handler_create("Capture");
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
