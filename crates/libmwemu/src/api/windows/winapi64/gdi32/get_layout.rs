use crate::emu;

pub fn GetLayout(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    log_red!(emu, "gdi32!GetLayout hdc:0x{:x}", hdc);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // LAYOUT_LTR (default left-to-right)
}
