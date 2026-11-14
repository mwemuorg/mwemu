use crate::emu;
use crate::winapi::helper;

pub fn abort(emu: &mut emu::Emu) {
    log_red!(emu, "msvcrt!abort");
    helper::advance_tick(emu, 1);
    emu.stop();
}
