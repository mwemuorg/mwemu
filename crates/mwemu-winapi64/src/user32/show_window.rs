use crate::emu;

pub(super) fn ShowWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!ShowWindow hwnd:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = 1;
}
