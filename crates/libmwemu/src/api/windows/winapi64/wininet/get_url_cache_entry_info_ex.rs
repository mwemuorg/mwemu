use crate::emu;

pub fn GetUrlCacheEntryInfoExA(emu: &mut emu::Emu) {
    get_url_cache_entry_info_ex(emu, "GetUrlCacheEntryInfoExA");
}

fn get_url_cache_entry_info_ex(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!{api} arg1:0x{:x}", arg1);
    emu.set_last_error(2);
    emu.regs_mut().rax = 0;
}

pub fn GetUrlCacheEntryInfoExW(emu: &mut emu::Emu) {
    get_url_cache_entry_info_ex(emu, "GetUrlCacheEntryInfoExW");
}
