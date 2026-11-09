use crate::emu;

pub(super) fn SetForegroundWindow(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "user32!SetForegroundWindow hwnd:0x{:x}", hwnd);
    emu.regs_mut().rax = 1;
}
