use crate::emu;
use crate::winapi::helper;

pub fn DeleteFiber(emu: &mut emu::Emu) {
    let fiber = emu.regs().rcx;
    log_red!(emu, "kernel32!DeleteFiber fiber:0x{:x}", fiber);
    helper::handler_close(fiber);
}
