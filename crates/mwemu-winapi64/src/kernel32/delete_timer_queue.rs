use crate::emu;
use crate::winapi::helper;

pub fn DeleteTimerQueue(emu: &mut emu::Emu) {
    let timer_queue = emu.regs().rcx;
    log_red!(emu, "kernel32!DeleteTimerQueue queue:0x{:x}", timer_queue);
    helper::handler_close(timer_queue);
    emu.regs_mut().rax = 1; // TRUE
}
