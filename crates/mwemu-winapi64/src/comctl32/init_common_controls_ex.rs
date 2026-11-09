use crate::emu;

pub(super) fn InitCommonControlsEx(emu: &mut emu::Emu) {
    log_red!(emu, "comctl32!InitCommonControlsEx");
    // TODO: do something
    emu.regs_mut().rax = 1; // TRUE
}
