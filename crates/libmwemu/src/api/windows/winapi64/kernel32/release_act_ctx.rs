
use crate::emu;
use crate::winapi::helper;
pub fn ReleaseActCtx(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    log_red!(emu, "kernel32!ReleaseActCtx param1:0x{:x}", param1);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
