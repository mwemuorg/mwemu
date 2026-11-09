use crate::emu;
use crate::winapi::helper;

pub fn CreatePopupMenu(emu: &mut emu::Emu) {
    log_red!(emu, "user32!CreatePopupMenu");
    emu.regs_mut().rax = helper::handler_create("CreatePopupMenu");
}
