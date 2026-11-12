use crate::emu;
use crate::winapi::helper;

pub fn UnlockUrlCacheEntryFile(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "wininet!UnlockUrlCacheEntryFile arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn UnlockUrlCacheEntryFileA(emu: &mut emu::Emu) {
    unlock_url_cache_entry_file(emu, "UnlockUrlCacheEntryFileA");
}

fn unlock_url_cache_entry_file(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "wininet!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn UnlockUrlCacheEntryFileW(emu: &mut emu::Emu) {
    unlock_url_cache_entry_file(emu, "UnlockUrlCacheEntryFileW");
}
