use crate::emu;
use crate::winapi::helper;

pub fn GetSubMenu(emu: &mut emu::Emu) {
    let hmenu = emu.regs().rcx;
    let npos = emu.regs().rdx;
    log_red!(emu, "user32!GetSubMenu menu:0x{:x} pos:{}", hmenu, npos);
    let hndl = helper::handler_create("user32://menu");
    emu.regs_mut().rax = hndl;
}
