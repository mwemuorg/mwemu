use crate::emu;

pub fn SetPixel(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let x = emu.regs().rdx as i32;
    let y = emu.regs().r8 as i32;
    let color = emu.regs().r9 as u32;
    log_red!(
        emu,
        "gdi32!SetPixel hdc:0x{:x} x:{} y:{} color:0x{:06x}",
        hdc,
        x,
        y,
        color
    );
    // Return the RGB color that was set (or CLR_INVALID on failure)
    emu.set_last_error(0);
    emu.regs_mut().rax = color as u64;
}
