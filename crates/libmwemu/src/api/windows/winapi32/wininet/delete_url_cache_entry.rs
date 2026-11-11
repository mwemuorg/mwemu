use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn DeleteUrlCacheEntry(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    log_red!(emu, "wininet!DeleteUrlCacheEntry handle:0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

pub fn DeleteUrlCacheEntryA(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    log_red!(emu, "wininet!DeleteUrlCacheEntryA handle:0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

pub fn DeleteUrlCacheEntryW(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    log_red!(emu, "wininet!DeleteUrlCacheEntryW handle:0x{:x}", handle);
    helper::handler_close(handle);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
