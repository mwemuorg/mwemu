use crate::emu;
use crate::winapi::helper;

pub fn CloseThreadpoolWait(emu: &mut emu::Emu) {
    let pwa = emu.regs().rcx;
    log_red!(emu, "kernel32!CloseThreadpoolWait pwa:0x{:x}", pwa);
    helper::handler_close(pwa);
    helper::advance_tick(emu, 1);
}
