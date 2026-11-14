use crate::emu;
use crate::winapi::helper;

pub fn SetBkMode(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let mode = emu.regs().rdx;
    let mode_name = match mode {
        1 => "TRANSPARENT",
        2 => "OPAQUE",
        _ => "UNKNOWN",
    };
    log_red!(
        emu,
        "gdi32!SetBkMode hdc:0x{:x} mode:{} ({})",
        hdc,
        mode,
        mode_name
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
