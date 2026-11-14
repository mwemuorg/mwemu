use crate::emu;
use crate::winapi::helper;

pub fn FindFirstUrlCacheEntryExA(emu: &mut emu::Emu) {
    find_first_url_cache_entry_ex(emu, "FindFirstUrlCacheEntryExA", false);
}

pub fn FindFirstUrlCacheEntryExW(emu: &mut emu::Emu) {
    find_first_url_cache_entry_ex(emu, "FindFirstUrlCacheEntryExW", true);
}

fn find_first_url_cache_entry_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    if wide {
        let handle = helper::handler_create("findfirsturlcacheentryexw");
        emu.regs_mut().rax = handle;
    } else {
        let handle = helper::handler_create("findfirsturlcacheentryexa");
        emu.regs_mut().rax = handle;
    }
}
