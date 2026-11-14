use crate::emu;

pub(super) fn IsWindowVisible(emu: &mut emu::Emu) {
    log_red!(emu, "user32!IsWindowVisible hwnd:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = 1;
}
