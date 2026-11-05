use crate::emu;
use crate::winapi::helper;

pub fn FindCloseChangeNotification(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernel32!FindCloseChangeNotification h:0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 1; // TRUE
}
