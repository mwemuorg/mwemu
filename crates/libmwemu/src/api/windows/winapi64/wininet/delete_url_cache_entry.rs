use crate::emu;
use crate::winapi::helper;

pub fn DeleteUrlCacheEntry(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    helper::handler_close(handle);
    log_red!(emu, "wininet!DeleteUrlCacheEntry handle:0x{:x}", handle);
    emu.regs_mut().rax = 1;
}

pub fn DeleteUrlCacheEntryA(emu: &mut emu::Emu) {
    delete_url_cache_entry(emu, "DeleteUrlCacheEntryA");
}

fn delete_url_cache_entry(emu: &mut emu::Emu, api: &str) {
    let handle = emu.regs().rcx;
    log_red!(emu, "wininet!{api} handle:0x{:x}", handle);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn DeleteUrlCacheEntryW(emu: &mut emu::Emu) {
    delete_url_cache_entry(emu, "DeleteUrlCacheEntryW");
}
