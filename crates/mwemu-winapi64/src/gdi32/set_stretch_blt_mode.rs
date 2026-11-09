use crate::emu;
use crate::winapi::helper;

pub fn SetStretchBltMode(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let mode = emu.regs().rdx;
    log_red!(emu, "gdi32!SetStretchBltMode hdc:0x{:x} mode:{}", hdc, mode);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
