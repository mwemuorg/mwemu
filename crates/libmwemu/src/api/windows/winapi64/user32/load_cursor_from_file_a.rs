use crate::emu;
use crate::winapi::helper;

pub fn LoadCursorFromFileA(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "user32!LoadCursorFromFileA arg1:0x{:x}", arg1);
    let hndl = helper::handler_create("user32://LoadCursorFromFileA");
    emu.regs_mut().rax = hndl;
}
