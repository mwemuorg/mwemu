use crate::emu;

pub fn SetLayout(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let layout = emu.regs().rdx;
    log_red!(emu, "gdi32!SetLayout hdc:0x{:x} layout:0x{:x}", hdc, layout);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // previous layout
}
