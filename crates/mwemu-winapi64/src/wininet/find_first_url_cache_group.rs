use crate::emu;
use crate::winapi::helper;

pub fn FindFirstUrlCacheGroup(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "wininet!FindFirstUrlCacheGroup arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("findfirsturlcachegroup");
    emu.regs_mut().rax = handle;
}
