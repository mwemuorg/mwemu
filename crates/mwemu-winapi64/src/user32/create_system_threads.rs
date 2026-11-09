use crate::emu;
use crate::winapi::helper;

pub fn CreateSystemThreads(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("CreateSystemThreads");
    log_red!(emu, "user32!CreateSystemThreads => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
