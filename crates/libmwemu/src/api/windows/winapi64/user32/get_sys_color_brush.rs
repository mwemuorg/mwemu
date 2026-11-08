use crate::emu;
use crate::winapi::helper;

pub fn GetSysColorBrush(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let hndl = helper::handler_create("brush");
    log_red!(emu, "user32!GetSysColorBrush => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
