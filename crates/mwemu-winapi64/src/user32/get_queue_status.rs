use crate::emu;
use crate::winapi::helper;

pub fn GetQueueStatus(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetQueueStatus");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
