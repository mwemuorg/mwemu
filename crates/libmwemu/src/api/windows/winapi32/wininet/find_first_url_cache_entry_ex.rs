use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn FindFirstUrlCacheEntryExA(emu: &mut emu::Emu) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(
        emu,
        "wininet!FindFirstUrlCacheEntryExA arg0:0x{:x} arg1:0x{:x}",
        a0,
        a1
    );
    emu.regs_mut().rax = helper::handler_create("findfirsturlcacheentryexa");
    stdcall_ret(emu, 10);
}

pub fn FindFirstUrlCacheEntryExW(emu: &mut emu::Emu) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(
        emu,
        "wininet!FindFirstUrlCacheEntryExW arg0:0x{:x} arg1:0x{:x}",
        a0,
        a1
    );
    emu.regs_mut().rax = helper::handler_create("findfirsturlcacheentryexw");
    stdcall_ret(emu, 10);
}
