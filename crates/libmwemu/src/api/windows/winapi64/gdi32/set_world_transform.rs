use crate::emu;
use crate::winapi::helper;

pub fn SetWorldTransform(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let xform = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!SetWorldTransform hdc:0x{:x} xform:0x{:x}",
        hdc,
        xform
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
