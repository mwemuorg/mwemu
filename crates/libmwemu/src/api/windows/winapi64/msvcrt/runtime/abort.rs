use crate::emu;

pub fn abort(emu: &mut emu::Emu) {
    log_red!(emu, "msvcrt!abort");
    emu.stop();
}
