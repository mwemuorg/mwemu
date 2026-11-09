use crate::emu;
use crate::winapi::helper;

pub fn GetTextAlign(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!GetTextAlign hdc:0x{:x}", hdc);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // TA_LEFT | TA_TOP
}
