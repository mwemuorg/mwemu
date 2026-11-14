use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateWellKnownSid(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0) as u32;
    let hndl = helper::handler_create("CreateWellKnownSid");
    log_red!(emu, "advapi32!CreateWellKnownSid => 0x{:x}", hndl);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 4);
}
