use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn DeleteTimerQueueTimer(emu: &mut emu::Emu) {
    let timer_queue = arg(emu, 0) as u64;
    let timer = arg(emu, 1) as u64;
    let completion_event = arg(emu, 2) as u64;
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
    stdcall_ret(emu, 3);
}
