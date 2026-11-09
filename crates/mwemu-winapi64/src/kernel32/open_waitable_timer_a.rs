use crate::emu;
use crate::winapi::helper;

pub fn OpenWaitableTimerA(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!OpenWaitableTimerA");
    emu.regs_mut().rax = helper::handler_create("OpenWaitableTimerA");
}
