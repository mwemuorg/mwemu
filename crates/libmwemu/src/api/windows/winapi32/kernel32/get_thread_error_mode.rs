use crate::emu;
use crate::winapi::helper;

pub fn get_thread_error_mode(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetThreadErrorMode");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
