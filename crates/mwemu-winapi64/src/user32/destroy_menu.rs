use crate::emu;
use crate::winapi::helper;

pub fn DestroyMenu(emu: &mut emu::Emu) {
    let hmenu = emu.regs().rcx;
    log_red!(emu, "user32!DestroyMenu hmenu:0x{:x}", hmenu);
    if hmenu != 0 && helper::handler_exist(hmenu) {
        helper::handler_close(hmenu);
    }
    emu.regs_mut().rax = 1;
}
