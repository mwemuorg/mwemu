use crate::emu;

pub(super) fn EnableWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!EnableWindow hwnd:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = 0;
}
