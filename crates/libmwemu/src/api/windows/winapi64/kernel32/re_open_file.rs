use crate::emu;
use crate::winapi::helper;

pub fn ReOpenFile(emu: &mut emu::Emu) {
    let orig_handle = emu.regs().rcx;
    let desired_access = emu.regs().rdx;
    let share_mode = emu.regs().r8;
    let flags = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!ReOpenFile h:0x{:x} access:0x{:x} share:0x{:x} flags:0x{:x}",
        orig_handle,
        desired_access,
        share_mode,
        flags
    );
    if helper::handler_exist(orig_handle) {
        let uri = helper::handler_get_uri(orig_handle).unwrap_or_default();
        emu.regs_mut().rax = helper::handler_create(&uri);
    } else {
        emu.regs_mut().rax = u64::MAX; // INVALID_HANDLE_VALUE
    }
}
