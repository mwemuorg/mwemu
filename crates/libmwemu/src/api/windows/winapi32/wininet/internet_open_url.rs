use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn InternetOpenUrlA(emu: &mut emu::Emu) {
    internet_open_url(emu, "InternetOpenUrlA", false);
}

pub fn InternetOpenUrlW(emu: &mut emu::Emu) {
    internet_open_url(emu, "InternetOpenUrlW", true);
}

fn internet_open_url(emu: &mut emu::Emu, api: &str, wide: bool) {
    let a0 = arg(emu, 0) as u64;
    let a1 = arg(emu, 1) as u64;
    log_red!(emu, "wininet!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    if wide {
        emu.regs_mut().rax = helper::handler_create("internetopenurlw");
    } else {
        emu.regs_mut().rax = helper::handler_create("internetopenurla");
    }

    stdcall_ret(emu, 6);
}
