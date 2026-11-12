use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateUrlCacheEntryA(emu: &mut emu::Emu) {
    create_url_cache_entry(emu, "CreateUrlCacheEntryA", false);
}

pub fn CreateUrlCacheEntryW(emu: &mut emu::Emu) {
    create_url_cache_entry(emu, "CreateUrlCacheEntryW", true);
}

fn create_url_cache_entry(emu: &mut emu::Emu, api: &str, wide: bool) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(emu, "wininet!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    if wide {
        emu.regs_mut().rax = helper::handler_create("createurlcacheentryw");
    } else {
        emu.regs_mut().rax = helper::handler_create("createurlcacheentrya");
    }

    stdcall_ret(emu, 5);
}
