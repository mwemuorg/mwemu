use crate::emu;
use crate::winapi::helper;

pub fn ClosePseudoConsole(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernel32!ClosePseudoConsole h:0x{:x}", handle);
    helper::handler_close(handle);
}
