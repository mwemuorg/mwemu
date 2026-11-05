use crate::emu;
use crate::winapi::helper;

pub fn CreateMemoryResourceNotification(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateMemoryResourceNotification");
    emu.regs_mut().rax = helper::handler_create("CreateMemoryResourceNotification");
}
