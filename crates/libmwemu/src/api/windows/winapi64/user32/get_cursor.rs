use crate::emu;
use crate::winapi::helper;

pub fn GetCursor(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetCursor");
    let hndl = helper::handler_create("user32://cursor");
    emu.regs_mut().rax = hndl;
}
