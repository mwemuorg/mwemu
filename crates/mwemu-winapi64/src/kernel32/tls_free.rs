use crate::emu;
use crate::winapi::helper;
pub fn TlsFree(emu: &mut emu::Emu) {
    let idx = emu.regs().rcx as usize; // First parameter passed in RCX in x64
    log_red!(emu, "kernel32!TlsFree idx: {}", idx);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    if idx < emu.tls64().len() {
        emu.tls64_mut()[idx] = 0; // Clear the slot
        emu.set_last_error(0);
        emu.regs_mut().rax = 1; // Return TRUE
    } else {
        emu.regs_mut().rax = 0; // Return FALSE if invalid index
    }
}
