use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateUrlCacheEntryA(emu: &mut emu::Emu) {
    create_url_cache_entry(emu, "CreateUrlCacheEntryA");
}

fn create_url_cache_entry(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = WinApiAbi::arg(emu, 4);
    log_red!(
        emu,
        "wininet!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    let handle = helper::handler_create(api);

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn CreateUrlCacheEntryW(emu: &mut emu::Emu) {
    create_url_cache_entry(emu, "CreateUrlCacheEntryW");
}
