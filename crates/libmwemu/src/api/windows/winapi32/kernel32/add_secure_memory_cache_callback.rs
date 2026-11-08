use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn AddSecureMemoryCacheCallback(emu: &mut emu::Emu) {
    let callback = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!AddSecureMemoryCacheCallback callback:0x{:x}",
        callback
    );
    let handle = helper::handler_create("AddSecureMemoryCacheCallback");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
