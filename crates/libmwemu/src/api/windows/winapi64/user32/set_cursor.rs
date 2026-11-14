use crate::emu;

pub(super) fn SetCursor(emu: &mut emu::Emu) {
    let hcursor = emu.regs().rcx;
    log_red!(emu, "user32!SetCursor hCursor:0x{:x}", hcursor);
    emu.regs_mut().rax = hcursor;
}
