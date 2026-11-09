use crate::emu;
use crate::winapi::helper;

pub fn LoadMenuA(emu: &mut emu::Emu) {
    let hinst = emu.regs().rcx;
    log_red!(emu, "user32!LoadMenuA hInst:0x{:x}", hinst);
    let hndl = helper::handler_create("user32://menu");
    emu.regs_mut().rax = hndl;
}
