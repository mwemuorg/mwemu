use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn DeleteTimerQueueEx(emu: &mut emu::Emu) {
    let timer_queue = arg(emu, 0) as u64;
    let completion_event = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!DeleteTimerQueueEx queue:0x{:x} event:0x{:x}",
        timer_queue,
        completion_event
    );
    helper::handler_close(timer_queue);
    helper::advance_tick(emu, 1);
    emu.regs_mut().rax = 1; // TRUE
    stdcall_ret(emu, 2);
}
