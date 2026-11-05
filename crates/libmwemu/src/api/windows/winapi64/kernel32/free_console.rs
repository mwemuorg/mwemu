use crate::emu;
use crate::winapi::helper;

pub fn FreeConsole(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernel32!FreeConsole handle:0x{:x}", handle);
    if handle != 0 {
        helper::handler_close(handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
