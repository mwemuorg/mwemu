use crate::emu;
use crate::winapi::helper;

pub fn GetClipboardData(emu: &mut emu::Emu) {
    let format = emu.regs().rcx;
    log_red!(emu, "user32!GetClipboardData format:{}", format);
    let handle = helper::handler_create("ClipboardData");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}
