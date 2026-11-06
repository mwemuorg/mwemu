
use crate::emu;
use crate::winapi::helper;
pub fn CloseConsoleHandle(emu: &mut emu::Emu) {
    let _a1 = emu.regs().rcx as u32;
    log_red!(emu, "kernel32!CloseConsoleHandle");
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
