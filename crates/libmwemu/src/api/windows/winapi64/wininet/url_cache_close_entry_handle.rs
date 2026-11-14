use crate::emu;
use crate::winapi::helper;

pub fn UrlCacheCloseEntryHandle(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(
        emu,
        "wininet!UrlCacheCloseEntryHandle handle:0x{:x}",
        handle
    );
    if handle != 0 {
        helper::handler_close(handle);
    }
    emu.regs_mut().rax = 1;
}
