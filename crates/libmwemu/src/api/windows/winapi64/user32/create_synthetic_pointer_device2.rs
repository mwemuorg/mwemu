use crate::emu;
use crate::winapi::helper;

pub fn CreateSyntheticPointerDevice2(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("CreateSyntheticPointerDevice2");
    log_red!(emu, "user32!CreateSyntheticPointerDevice2 => 0x{:x}", hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
