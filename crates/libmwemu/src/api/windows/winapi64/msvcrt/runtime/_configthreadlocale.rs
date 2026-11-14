use crate::emu;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _configthreadlocale(emu: &mut emu::Emu) {
    let per_thread = emu.regs().rcx as i32;
    log_red!(emu, "msvcrt!_configthreadlocale type:{}", per_thread);
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, per_thread as u32);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // _ENABLE_PER_THREAD_LOCALE_OLD = 0
}
