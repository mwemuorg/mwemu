use crate::emu;
use crate::winapi::helper;

pub fn CreateMenu(emu: &mut emu::Emu) {
    log_red!(emu, "user32!CreateMenu");
    emu.regs_mut().rax = helper::handler_create("CreateMenu");
}
