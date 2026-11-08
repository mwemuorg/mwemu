use crate::emu;
use crate::winapi::helper;

pub fn CreateTimerQueue_api(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateTimerQueue");
    emu.regs_mut().rax = helper::handler_create("CreateTimerQueue");
}
