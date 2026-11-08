use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateMemoryResourceNotification(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateMemoryResourceNotification");
    emu.regs_mut().rax = helper::handler_create("CreateMemoryResourceNotification");
    stdcall_ret(emu, 1);
}
