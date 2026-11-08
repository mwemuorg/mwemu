use crate::emu;
use crate::winapi::helper;
pub fn AddSecureMemoryCacheCallback(emu: &mut emu::Emu) {
    let callback = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!AddSecureMemoryCacheCallback callback:0x{:x}",
        callback
    );
    let handle = helper::handler_create("AddSecureMemoryCacheCallback");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
