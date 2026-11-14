use crate::emu;

pub(super) fn IsAppThemed(emu: &mut emu::Emu) {
    log_red!(emu, "** {} uxtheme!IsAppThemed", emu.pos);
    emu.regs_mut().rax = 1;
}
