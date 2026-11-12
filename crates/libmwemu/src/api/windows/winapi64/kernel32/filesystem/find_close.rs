use crate::emu;
use crate::winapi::helper;

pub fn FindClose(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;

    log_red!(emu, "kernel32!FindClose");
    helper::handler_close(hndl);
    emu.regs_mut().rax = 1;
}
