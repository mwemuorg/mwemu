use crate::emu;

pub(super) fn SetFocus(emu: &mut emu::Emu) {
    log_red!(emu, "user32!SetFocus hwnd:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = emu.regs().rcx;
}
