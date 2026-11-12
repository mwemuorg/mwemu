use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreatePrivateObjectSecurityEx(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let hndl = helper::handler_create("CreatePrivateObjectSecurityEx");
    log_red!(
        emu,
        "advapi32!CreatePrivateObjectSecurityEx => 0x{:x}",
        hndl
    );
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 8);
}
