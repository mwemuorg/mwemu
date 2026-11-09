use crate::emu;
use crate::winapi::helper;
pub fn FreeMemoryJobObject(emu: &mut emu::Emu) {
    let _Buffer = emu.regs().rcx as u32;
    log_red!(emu, "kernel32!FreeMemoryJobObject");
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
