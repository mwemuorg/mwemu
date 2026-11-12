use crate::emu;

pub fn FindNextUrlCacheContainerA(emu: &mut emu::Emu) {
    find_next_url_cache_container(emu, "FindNextUrlCacheContainerA");
}

fn find_next_url_cache_container(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!{api} arg1:0x{:x}", arg1);
    emu.set_last_error(2);
    emu.regs_mut().rax = 0;
}

pub fn FindNextUrlCacheContainerW(emu: &mut emu::Emu) {
    find_next_url_cache_container(emu, "FindNextUrlCacheContainerW");
}
