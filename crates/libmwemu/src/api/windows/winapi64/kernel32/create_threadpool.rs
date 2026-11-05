use crate::emu;
use crate::winapi::helper;

pub fn CreateThreadpool(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateThreadpool");
    emu.regs_mut().rax = helper::handler_create("CreateThreadpool");
}
