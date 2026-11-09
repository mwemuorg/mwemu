use crate::emu;
use crate::winapi::helper;

pub fn SetTextAlign(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let align = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!SetTextAlign hdc:0x{:x} align:0x{:x}",
        hdc,
        align
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // previous alignment (TA_LEFT | TA_TOP)
}
