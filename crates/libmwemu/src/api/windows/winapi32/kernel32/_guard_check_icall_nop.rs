use crate::emu;
use crate::winapi::helper;

pub fn _guard_check_icall_nop(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!_guard_check_icall_nop");
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
}
