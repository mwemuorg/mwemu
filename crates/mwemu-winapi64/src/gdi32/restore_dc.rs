use crate::emu;

pub fn RestoreDC(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let saved = emu.regs().rdx as i32;
    log_red!(emu, "gdi32!RestoreDC hdc:0x{:x} saved:{}", hdc, saved);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
