use crate::emu;
use crate::winapi::helper;

pub fn CreateServiceEx(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("CreateServiceEx");
    log_red!(emu, "advapi32!CreateServiceEx => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
