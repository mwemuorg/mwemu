use crate::emu;
use crate::winapi::helper;

pub fn CreateThreadpoolCleanupGroup(emu: &mut emu::Emu) {
    log_red!(emu, "kernelbase!CreateThreadpoolCleanupGroup");
    let hndl = helper::handler_create("threadpool_cleanup_group");
    emu.regs_mut().rax = hndl;
}
