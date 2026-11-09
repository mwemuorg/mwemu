use crate::emu;

pub fn GetNearestColor(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let color = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!GetNearestColor hdc:0x{:x} color:0x{:06x}",
        hdc,
        color
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = color & 0x00ff_ffff; // nearest == exact in 24-bit
}
