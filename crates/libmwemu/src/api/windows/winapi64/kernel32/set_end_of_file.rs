use crate::emu;
use crate::winapi::helper;

pub fn SetEndOfFile(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(emu, "kernel32!SetEndOfFile h:0x{:x}", handle);
    if helper::handler_exist(handle) {
        helper::handler_set_eof(handle);
    }
    emu.regs_mut().rax = 1;
}
