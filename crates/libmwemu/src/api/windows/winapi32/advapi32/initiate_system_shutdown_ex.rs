use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn InitiateSystemShutdownExA(emu: &mut emu::Emu) {
    initiate_system_shutdown_ex(emu, "InitiateSystemShutdownExA", false);
}

fn initiate_system_shutdown_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x}", s, p1);
    stdcall_ret(emu, 6);
    emu.regs_mut().rax = 1;
}

pub fn InitiateSystemShutdownExW(emu: &mut emu::Emu) {
    initiate_system_shutdown_ex(emu, "InitiateSystemShutdownExW", true);
}
