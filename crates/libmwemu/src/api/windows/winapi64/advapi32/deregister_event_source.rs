use crate::emu;
use crate::winapi::helper;

pub fn DeregisterEventSource(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    if helper::handler_exist(hndl) {
        helper::handler_close(hndl);
    }
    log_red!(emu, "advapi32!DeregisterEventSource 0x{:x}", hndl);
    emu.regs_mut().rax = 1;
}
