use crate::emu;

pub(super) fn IsWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!IsWindow hwnd:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = 1;
}
