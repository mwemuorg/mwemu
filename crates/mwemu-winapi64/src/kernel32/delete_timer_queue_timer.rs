use crate::emu;
use crate::winapi::helper;

pub fn DeleteTimerQueueTimer(emu: &mut emu::Emu) {
    let timer_queue = emu.regs().rcx;
    let timer = emu.regs().rdx;
    let completion_event = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!DeleteTimerQueueTimer queue:0x{:x} timer:0x{:x} event:0x{:x}",
        timer_queue,
        timer,
        completion_event
    );
    helper::handler_close(timer);
    helper::advance_tick(emu, 1);
    emu.regs_mut().rax = 1; // TRUE
}
