use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateRestrictedToken(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let hndl = helper::handler_create("CreateRestrictedToken");
    log_red!(emu, "advapi32!CreateRestrictedToken => 0x{:x}", hndl);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 9);
}
