use super::allocate_memory;
use crate::emu;

pub fn abort(emu: &mut emu::Emu) {
    log_red!(emu, "msvcrt!abort");
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, 3); // SIGABRT
    }
    emu.stop();
}
