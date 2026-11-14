use crate::emu;

pub fn GetUrlCacheGroupAttributeA(emu: &mut emu::Emu) {
    get_url_cache_group_attribute(emu, "GetUrlCacheGroupAttributeA");
}

fn get_url_cache_group_attribute(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!{api} arg1:0x{:x}", arg1);
    emu.set_last_error(2);
    emu.regs_mut().rax = 0;
}

pub fn GetUrlCacheGroupAttributeW(emu: &mut emu::Emu) {
    get_url_cache_group_attribute(emu, "GetUrlCacheGroupAttributeW");
}
