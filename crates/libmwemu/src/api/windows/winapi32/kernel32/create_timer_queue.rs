use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateTimerQueue(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateTimerQueue");
    emu.regs_mut().rax = helper::handler_create("CreateTimerQueue");
    stdcall_ret(emu, 1);
}
