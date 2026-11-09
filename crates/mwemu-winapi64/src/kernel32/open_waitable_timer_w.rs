use crate::emu;
use crate::winapi::helper;

pub fn OpenWaitableTimerW(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenWaitableTimerW");
    emu.regs_mut().rax = helper::handler_create("OpenWaitableTimerW");
}
