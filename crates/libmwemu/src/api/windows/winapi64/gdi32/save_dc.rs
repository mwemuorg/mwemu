use crate::emu;
use crate::winapi::helper;

pub fn SaveDC(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!SaveDC hdc:0x{:x}", hdc);
    if helper::handler_exist(hdc) {
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
    }
}
