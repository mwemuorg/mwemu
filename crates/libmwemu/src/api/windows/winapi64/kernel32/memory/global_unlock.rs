use crate::emu;
use crate::winapi::helper;
pub fn GlobalUnlock(emu: &mut emu::Emu) {
    let mem = emu.regs().rcx;
    log_red!(emu, "kernel32!GlobalUnlock mem: 0x{:x}", mem);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
