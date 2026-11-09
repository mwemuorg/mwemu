use crate::emu;
use crate::winapi::helper;

pub fn CreateThreadpoolCleanupGroup(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!CreateThreadpoolCleanupGroup");
    emu.regs_mut().rax = helper::handler_create("CreateThreadpoolCleanupGroup");
}
