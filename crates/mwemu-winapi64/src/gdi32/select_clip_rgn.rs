use crate::emu;

pub fn SelectClipRgn(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let hrgn = emu.regs().rdx;
    log_red!(emu, "gdi32!SelectClipRgn hdc:0x{:x} hrgn:0x{:x}", hdc, hrgn);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // SIMPLEREGION
}
