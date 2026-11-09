use crate::emu;
use crate::winapi::helper;

pub fn StartDoc(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let docinfo = emu.regs().rdx;
    log_red!(
        emu,
        "gdi32!StartDoc hdc:0x{:x} docinfo:0x{:x}",
        hdc,
        docinfo
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // job id
}
