use crate::emu;
use crate::winapi::helper;

pub fn OpenThreadDesktop(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("OpenThreadDesktop");
    log_red!(emu, "user32!OpenThreadDesktop => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
