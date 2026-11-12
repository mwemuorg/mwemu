use crate::emu;
use crate::winapi::helper;

pub fn FindFirstUrlCacheContainerA(emu: &mut emu::Emu) {
    find_first_url_cache_container(emu, "FindFirstUrlCacheContainerA", false);
}

pub fn FindFirstUrlCacheContainerW(emu: &mut emu::Emu) {
    find_first_url_cache_container(emu, "FindFirstUrlCacheContainerW", true);
}

fn find_first_url_cache_container(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    if wide {
        let handle = helper::handler_create("findfirsturlcachecontainerw");
        emu.regs_mut().rax = handle;
    } else {
        let handle = helper::handler_create("findfirsturlcachecontainera");
        emu.regs_mut().rax = handle;
    }
}
