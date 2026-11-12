use crate::emu;

pub fn GetUrlCacheEntryInfoA(emu: &mut emu::Emu) {
    get_url_cache_entry_info(emu, "GetUrlCacheEntryInfoA");
}

fn get_url_cache_entry_info(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!{api} arg1:0x{:x}", arg1);
    emu.set_last_error(2);
    emu.regs_mut().rax = 0;
}

pub fn GetUrlCacheEntryInfoW(emu: &mut emu::Emu) {
    get_url_cache_entry_info(emu, "GetUrlCacheEntryInfoW");
}
