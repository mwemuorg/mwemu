use crate::emu;
use crate::winapi::common::strings;

pub fn SetUrlCacheGroupAttributeA(emu: &mut emu::Emu) {
    set_url_cache_group_attribute(emu, "SetUrlCacheGroupAttributeA", false);
}

fn set_url_cache_group_attribute(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = strings::read_text(emu, val_ptr, wide);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetUrlCacheGroupAttributeW(emu: &mut emu::Emu) {
    set_url_cache_group_attribute(emu, "SetUrlCacheGroupAttributeW", true);
}
