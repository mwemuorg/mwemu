use crate::emu;
use crate::winapi::helper;

pub fn DeleteObject(emu: &mut emu::Emu) {
    let obj = emu.regs().rcx;
    log_red!(emu, "gdi32!DeleteObject obj:0x{:x}", obj);
    if helper::handler_exist(obj) {
        helper::handler_close(obj);
    }
    emu.regs_mut().rax = 1;
}
