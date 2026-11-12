use crate::emu;
use crate::winapi::common::strings;

pub fn FreeUrlCacheSpaceA(emu: &mut emu::Emu) {
    free_url_cache_space(emu, "FreeUrlCacheSpaceA", false);
}

fn free_url_cache_space(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "wininet!{api} '{}'", s);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn FreeUrlCacheSpaceW(emu: &mut emu::Emu) {
    free_url_cache_space(emu, "FreeUrlCacheSpaceW", true);
}
