use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ReOpenFile(emu: &mut emu::Emu) {
    let orig_handle = arg(emu, 0) as u64;
    let desired_access = arg(emu, 1) as u64;
    let share_mode = arg(emu, 2) as u64;
    let flags = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!ReOpenFile h:0x{:x} access:0x{:x} share:0x{:x} flags:0x{:x}",
        orig_handle,
        desired_access,
        share_mode,
        flags
    );
    if helper::handler_exist(orig_handle) {
        let uri = helper::handler_get_uri(orig_handle);
        emu.regs_mut().rax = helper::handler_create(&uri);
    } else {
        emu.regs_mut().rax = u64::MAX; // INVALID_HANDLE_VALUE
    }
    stdcall_ret(emu, 4);
}
