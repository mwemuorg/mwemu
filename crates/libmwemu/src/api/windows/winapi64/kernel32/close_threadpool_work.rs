use crate::emu;
use crate::winapi::helper;

pub fn CloseThreadpoolWork(emu: &mut emu::Emu) {
    let work = emu.regs().rcx;
    log_red!(emu, "kernel32!CloseThreadpoolWork work:0x{:x}", work);
    helper::handler_close(work);
}
