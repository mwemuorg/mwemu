use crate::emu;
use crate::winapi::helper;

pub fn CreateCaret(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let hndl = helper::handler_create("CreateCaret");
    log_red!(emu, "user32!CreateCaret => 0x{:x}", hndl);
    emu.regs_mut().rax = hndl;
}
