use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateUrlCacheEntryA(emu: &mut emu::Emu) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(
        emu,
        "wininet!CreateUrlCacheEntryA arg0:0x{:x} arg1:0x{:x}",
        a0,
        a1
    );
    emu.regs_mut().rax = helper::handler_create("createurlcacheentrya");
    stdcall_ret(emu, 6);
}

pub fn CreateUrlCacheEntryW(emu: &mut emu::Emu) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(
        emu,
        "wininet!CreateUrlCacheEntryW arg0:0x{:x} arg1:0x{:x}",
        a0,
        a1
    );
    emu.regs_mut().rax = helper::handler_create("createurlcacheentryw");
    stdcall_ret(emu, 6);
}
