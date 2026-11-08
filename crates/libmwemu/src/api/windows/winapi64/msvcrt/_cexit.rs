use super::allocate_memory;
use crate::emu;
use crate::winapi::helper;

pub fn _cexit(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "msvcrt!{} cleaning up", api);
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, 1); // cleanup flag
    }
    helper::advance_tick(emu, 1);
}
