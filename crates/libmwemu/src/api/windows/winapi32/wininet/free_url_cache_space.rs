use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn FreeUrlCacheSpaceA(emu: &mut emu::Emu) {
    free_url_cache_space(emu, "FreeUrlCacheSpaceA");
}

fn free_url_cache_space(emu: &mut emu::Emu, api: &str) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(emu, "wininet!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

pub fn FreeUrlCacheSpaceW(emu: &mut emu::Emu) {
    free_url_cache_space(emu, "FreeUrlCacheSpaceW");
}
