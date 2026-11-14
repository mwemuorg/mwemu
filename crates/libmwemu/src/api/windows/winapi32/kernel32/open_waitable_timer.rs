use crate::emu;
use crate::winapi::helper;

pub fn OpenWaitableTimerA(emu: &mut emu::Emu) {
    open_waitable_timer(emu, "OpenWaitableTimerA");
}

fn open_waitable_timer(emu: &mut emu::Emu, api: &str) {
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!{api}");
    emu.regs_mut().rax = helper::handler_create(api);
}

pub fn OpenWaitableTimerW(emu: &mut emu::Emu) {
    open_waitable_timer(emu, "OpenWaitableTimerW");
}
