use crate::emu;

pub(super) fn EndPaint(emu: &mut emu::Emu) {
    log_red!(emu, "user32!EndPaint");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
