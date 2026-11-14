use crate::emu;

pub(super) fn UpdateWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!UpdateWindow hwnd:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = 1;
}
