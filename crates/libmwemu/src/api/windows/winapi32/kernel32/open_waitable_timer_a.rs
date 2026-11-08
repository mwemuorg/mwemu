use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn OpenWaitableTimerA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenWaitableTimerA");
    emu.regs_mut().rax = helper::handler_create("OpenWaitableTimerA");
    stdcall_ret(emu, 1);
}
