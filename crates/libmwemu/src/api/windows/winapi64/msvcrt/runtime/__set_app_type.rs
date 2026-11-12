use crate::emu;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn __set_app_type(emu: &mut emu::Emu) {
    let app_type = emu.regs().rcx as u32;
    log_red!(emu, "msvcrt!__set_app_type  app_type: 0x{:x}", app_type);
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, app_type);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
