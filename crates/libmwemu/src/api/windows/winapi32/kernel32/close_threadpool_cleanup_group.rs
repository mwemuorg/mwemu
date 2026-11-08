use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CloseThreadpoolCleanupGroup(emu: &mut emu::Emu) {
    let group = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!CloseThreadpoolCleanupGroup group:0x{:x}",
        group
    );
    helper::handler_close(group);
    stdcall_ret(emu, 1);
}
