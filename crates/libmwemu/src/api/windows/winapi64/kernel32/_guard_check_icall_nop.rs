use crate::emu;

pub fn _guard_check_icall_nop(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!_guard_check_icall_nop");
    emu.set_last_error(0);
}
