use super::allocate_memory;
use crate::emu;

pub fn _endthread(emu: &mut emu::Emu, api: &str) {
    log_red!(emu, "msvcrt!{}", api);
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, 0); // exit code
    }
    emu.stop();
}
