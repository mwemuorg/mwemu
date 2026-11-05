use crate::emu;
use crate::winapi::helper;

pub fn FindNextChangeNotification(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernel32!FindNextChangeNotification h:0x{:x}", handle);
    if helper::handler_exist(handle) {
        emu.set_last_error(0);
        emu.regs_mut().rax = 1; // TRUE
    } else {
        emu.regs_mut().rax = 0; // FALSE
    }
}
