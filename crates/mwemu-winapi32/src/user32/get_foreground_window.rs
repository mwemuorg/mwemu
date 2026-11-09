use crate::emu;

pub(super) fn GetForegroundWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetForegroundWindow");
    emu.regs_mut().rax = 0x1337_0001;
}
