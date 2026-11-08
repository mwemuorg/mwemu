use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CloseThreadpoolCleanupGroupMembers(emu: &mut emu::Emu) {
    let group = arg(emu, 0) as u64;
    let cancel_pending = arg(emu, 1) as u64;
    let cleanup_context = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!CloseThreadpoolCleanupGroupMembers group:0x{:x} cancel:{} ctx:0x{:x}",
        group,
        cancel_pending != 0,
        cleanup_context
    );
    helper::handler_close(group);
    stdcall_ret(emu, 3);
}
