use crate::emu;
use crate::winapi::helper;

pub fn IsClipboardFormatAvailable(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    log_red!(emu, "user32!IsClipboardFormatAvailable");
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
