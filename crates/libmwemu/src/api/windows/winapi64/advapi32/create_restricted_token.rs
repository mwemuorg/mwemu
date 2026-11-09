use crate::emu;
use crate::winapi::helper;

pub fn CreateRestrictedToken(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("CreateRestrictedToken");
    log_red!(emu, "advapi32!CreateRestrictedToken => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
