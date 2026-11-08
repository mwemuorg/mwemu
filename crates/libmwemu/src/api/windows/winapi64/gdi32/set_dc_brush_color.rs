use crate::emu;

pub fn SetDCBrushColor(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let color = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!SetDCBrushColor hdc:0x{:x} color:0x{:06x}",
        hdc,
        color
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x00ff_ffff; // previous color (white)
}
