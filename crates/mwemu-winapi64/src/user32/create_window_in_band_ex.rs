use crate::emu;
use crate::winapi::helper;

pub fn CreateWindowInBandEx(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("CreateWindowInBandEx");
    log_red!(emu, "user32!CreateWindowInBandEx => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
