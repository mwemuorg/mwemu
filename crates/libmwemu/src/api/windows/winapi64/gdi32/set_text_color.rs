use crate::emu;
use crate::winapi::helper;

pub fn SetTextColor(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let color = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!SetTextColor hdc:0x{:x} color:0x{:06x}",
        hdc,
        color
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
