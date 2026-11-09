use crate::emu;
use crate::winapi::helper;

pub fn DeleteTimerQueueEx(emu: &mut emu::Emu) {
    let timer_queue = emu.regs().rcx;
    let completion_event = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!DeleteTimerQueueEx queue:0x{:x} event:0x{:x}",
        timer_queue,
        completion_event
    );
    helper::handler_close(timer_queue);
    helper::advance_tick(emu, 1);
    emu.regs_mut().rax = 1; // TRUE
}
