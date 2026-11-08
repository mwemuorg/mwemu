use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateThreadpoolCleanupGroup(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateThreadpoolCleanupGroup");
    emu.regs_mut().rax = helper::handler_create("CreateThreadpoolCleanupGroup");
    stdcall_ret(emu, 1);
}
