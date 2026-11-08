use crate::emu;
use crate::winapi::helper;

pub fn CreateCodeAuthzLevel(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("CreateCodeAuthzLevel");
    log_red!(emu, "advapi32!CreateCodeAuthzLevel => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
