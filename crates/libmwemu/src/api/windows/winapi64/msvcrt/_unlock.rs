use super::allocate_memory;
use crate::emu;
use crate::winapi::helper;

pub fn _unlock(emu: &mut emu::Emu) {
    let lock_num = emu.regs().rcx as u32;
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, lock_num);
    }
    log_red!(emu, "msvcrt!_unlock {}", lock_num);
    helper::advance_tick(emu, 1);
}
