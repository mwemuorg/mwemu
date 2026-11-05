use crate::emu;
use crate::winapi::helper;

pub fn GetConsoleCP(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetConsoleCP");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 437;
}
