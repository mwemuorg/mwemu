use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateTimerQueueTimer(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateTimerQueueTimer");
    emu.regs_mut().rax = helper::handler_create("CreateTimerQueueTimer");
    stdcall_ret(emu, 1);
}
