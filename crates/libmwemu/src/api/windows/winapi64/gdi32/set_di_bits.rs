use crate::emu;
use crate::winapi::helper;

pub fn SetDIBits(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let hbmp = emu.regs().rdx;
    let start = emu.regs().r8;
    let lines = emu.regs().r9;
    log_red!(
        emu,
        "gdi32!SetDIBits hdc:0x{:x} hbmp:0x{:x} start:{} lines:{}",
        hdc,
        hbmp,
        start,
        lines
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = lines;
}
