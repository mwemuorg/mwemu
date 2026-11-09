use crate::emu;

pub(super) fn IsThemeActive(emu: &mut emu::Emu) {
    log_red!(emu, "** {} uxtheme!IsThemeActive", emu.pos);
    emu.regs_mut().rax = 1;
}
