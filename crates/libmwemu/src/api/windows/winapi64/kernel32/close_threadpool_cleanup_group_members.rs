use crate::emu;
use crate::winapi::helper;

pub fn CloseThreadpoolCleanupGroupMembers(emu: &mut emu::Emu) {
    let group = emu.regs().rcx;
    let cancel_pending = emu.regs().rdx;
    let cleanup_context = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!CloseThreadpoolCleanupGroupMembers group:0x{:x} cancel:{} ctx:0x{:x}",
        group,
        cancel_pending != 0,
        cleanup_context
    );
    helper::handler_close(group);
}
