use crate::emu;

pub fn LZDone(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!LZDone");
    emu.set_last_error(0);
}
