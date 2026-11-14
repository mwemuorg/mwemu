use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn UnlockUrlCacheEntryFile(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    log_red!(emu, "wininet!UnlockUrlCacheEntryFile handle:0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

pub fn UnlockUrlCacheEntryFileA(emu: &mut emu::Emu) {
    unlock_url_cache_entry_file(emu, "UnlockUrlCacheEntryFileA");
}

fn unlock_url_cache_entry_file(emu: &mut emu::Emu, api: &str) {
    let handle = arg(emu, 0) as u64;
    log_red!(emu, "wininet!{api} handle:0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

pub fn UnlockUrlCacheEntryFileW(emu: &mut emu::Emu) {
    unlock_url_cache_entry_file(emu, "UnlockUrlCacheEntryFileW");
}
