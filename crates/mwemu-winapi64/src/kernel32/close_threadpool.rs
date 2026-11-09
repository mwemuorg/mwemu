use crate::emu;
use crate::winapi::helper;

pub fn CloseThreadpool(emu: &mut emu::Emu) {
    let pool = emu.regs().rcx;
    log_red!(emu, "kernel32!CloseThreadpool pool:0x{:x}", pool);
    helper::handler_close(pool);
}
