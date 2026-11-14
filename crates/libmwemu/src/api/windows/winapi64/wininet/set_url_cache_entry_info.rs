use crate::emu;

pub fn SetUrlCacheEntryInfoA(emu: &mut emu::Emu) {
    set_url_cache_entry_info(emu, "SetUrlCacheEntryInfoA");
}

fn set_url_cache_entry_info(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "wininet!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SetUrlCacheEntryInfoW(emu: &mut emu::Emu) {
    set_url_cache_entry_info(emu, "SetUrlCacheEntryInfoW");
}
