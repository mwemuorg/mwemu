use crate::emu;

pub(super) fn GetThemeAppProperties(emu: &mut emu::Emu) {
    log_red!(emu, "** {} uxtheme!GetThemeAppProperties", emu.pos);
    emu.regs_mut().rax = 1;
}
