use crate::emu;

pub(super) fn GetFocus(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetFocus");
    emu.regs_mut().rax = 0x1337_0001;
}
