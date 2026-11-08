use crate::emu;
use crate::winapi::helper;

pub fn DeleteDC(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!DeleteDC hdc:0x{:x}", hdc);
    if helper::handler_exist(hdc) {
        helper::handler_close(hdc);
    }
    emu.regs_mut().rax = 1;
}
