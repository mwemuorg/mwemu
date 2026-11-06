
use crate::emu;
use crate::winapi::helper;
pub fn GlobalDeleteAtom(emu: &mut emu::Emu) {
    let atom = emu.regs().rcx as u16;
    log_red!(emu, "kernel32!GlobalDeleteAtom atom:0x{:x}", atom);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
