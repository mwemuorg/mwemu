use crate::emu;
use crate::winapi::helper;

pub fn CreateWindowInBand(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("CreateWindowInBand");
    log_red!(emu, "user32!CreateWindowInBand => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
