use crate::emu;

pub fn TimeInit(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!TimeInit ",);
    emu.set_last_error(0);
}
