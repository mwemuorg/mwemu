use crate::emu;
use crate::winapi::helper;

pub fn _cexit(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "msvcrt!{} cleaning up", api);
    helper::advance_tick(emu, 1);
}
