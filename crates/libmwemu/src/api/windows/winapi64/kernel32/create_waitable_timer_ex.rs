use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::helper;

pub fn CreateWaitableTimerExA(emu: &mut Emu) {
    invoke(emu, false, "CreateWaitableTimerExA");
}

pub fn CreateWaitableTimerExW(emu: &mut Emu) {
    invoke(emu, true, "CreateWaitableTimerExW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let name = strings::read_text(emu, emu.regs().rdx, wide);
    log_red!(emu, "kernel32!{} name:'{}'", api, name);
    emu.regs_mut().rax = helper::handler_create("waitable_timer");
}
