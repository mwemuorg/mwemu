use crate::emu;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _lock(emu: &mut emu::Emu) {
    let lock_num = emu.regs().rcx as u32;
    log_red!(emu, "msvcrt!_lock {}", lock_num);
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, lock_num);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
