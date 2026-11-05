use crate::emu;
use crate::winapi::helper;

pub fn ResetEvent(emu: &mut emu::Emu) {
    let event = emu.regs().rcx;
    log_red!(emu, "kernel32!ResetEvent event:0x{:x}", event);
    if helper::handler_exist(event) {
        emu.regs_mut().rax = 1; // TRUE
    } else {
        emu.regs_mut().rax = 0; // FALSE
        emu.set_last_error(6); // ERROR_INVALID_HANDLE
    }
}
