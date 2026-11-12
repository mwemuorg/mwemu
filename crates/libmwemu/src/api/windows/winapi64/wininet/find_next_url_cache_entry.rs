use crate::emu;

pub fn FindNextUrlCacheEntryA(emu: &mut emu::Emu) {
    find_next_url_cache_entry(emu, "FindNextUrlCacheEntryA");
}

fn find_next_url_cache_entry(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!{api} arg1:0x{:x}", arg1);
    emu.set_last_error(2);
    emu.regs_mut().rax = 0;
}

pub fn FindNextUrlCacheEntryW(emu: &mut emu::Emu) {
    find_next_url_cache_entry(emu, "FindNextUrlCacheEntryW");
}
