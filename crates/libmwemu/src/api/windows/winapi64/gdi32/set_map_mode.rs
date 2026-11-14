use crate::emu;

pub fn SetMapMode(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let mode = emu.regs().rdx;
    let mode_name = match mode {
        1 => "MM_TEXT",
        2 => "MM_LOMETRIC",
        3 => "MM_HIMETRIC",
        4 => "MM_LOENGLISH",
        5 => "MM_HIENGLISH",
        6 => "MM_TWIPS",
        7 => "MM_ISOTROPIC",
        8 => "MM_ANISOTROPIC",
        _ => "UNKNOWN",
    };
    log_red!(
        emu,
        "gdi32!SetMapMode hdc:0x{:x} mode:{} ({})",
        hdc,
        mode,
        mode_name
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // previous mode: MM_TEXT
}
