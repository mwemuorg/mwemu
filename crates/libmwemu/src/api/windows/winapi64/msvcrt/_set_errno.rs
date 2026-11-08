use super::allocate_memory;
use crate::emu;

pub fn _set_errno(emu: &mut emu::Emu) {
    let err_value = emu.regs().rcx as u32;
    log_red!(emu, "msvcrt!_set_errno value:{}", err_value);
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, err_value);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // success
}
