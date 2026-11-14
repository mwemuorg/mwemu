use crate::emu;
use crate::winapi::helper;

pub fn IsGUIThread(emu: &mut emu::Emu) {
    log_red!(emu, "user32!IsGUIThread");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
