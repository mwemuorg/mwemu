use crate::emu;
use crate::winapi::helper;

pub fn _endthread(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "msvcrt!{}", api);
    helper::advance_tick(emu, 1);
    emu.stop();
}
