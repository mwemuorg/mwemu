use crate::emu;
use crate::winapi::helper;

pub fn CloseThreadpoolTimer(emu: &mut emu::Emu) {
    let timer = emu.regs().rcx;
    log_red!(emu, "kernel32!CloseThreadpoolTimer timer:0x{:x}", timer);
    helper::handler_close(timer);
    helper::advance_tick(emu, 1);
}
