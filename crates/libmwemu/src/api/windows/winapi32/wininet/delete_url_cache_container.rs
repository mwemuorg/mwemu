use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn DeleteUrlCacheContainerA(emu: &mut emu::Emu) {
    delete_url_cache_container(emu, "DeleteUrlCacheContainerA");
}

fn delete_url_cache_container(emu: &mut emu::Emu, api: &str) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(emu, "wininet!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

pub fn DeleteUrlCacheContainerW(emu: &mut emu::Emu) {
    delete_url_cache_container(emu, "DeleteUrlCacheContainerW");
}
