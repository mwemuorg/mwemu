use crate::emu;
use crate::winapi::helper;

pub fn CreateCompatibleDC(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!CreateCompatibleDC hdc:0x{:x}", hdc);
    emu.regs_mut().rax = helper::handler_create("gdi32://CreateCompatibleDC");
}
