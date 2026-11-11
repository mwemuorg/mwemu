use crate::emu;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _set_app_type(emu: &mut emu::Emu) {
    let app_type = emu.regs().rcx;
    log_red!(emu, "msvcrt!_set_app_type type:{}", app_type);
    let slot = allocate_memory(emu, 8).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_qword(slot, app_type);
    }
}
