use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateUrlCacheContainerA(emu: &mut emu::Emu) {
    create_url_cache_container(emu, "CreateUrlCacheContainerA");
}

fn create_url_cache_container(emu: &mut emu::Emu, api: &str) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(emu, "wininet!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 8);
}

pub fn CreateUrlCacheContainerW(emu: &mut emu::Emu) {
    create_url_cache_container(emu, "CreateUrlCacheContainerW");
}
