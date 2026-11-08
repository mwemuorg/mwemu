use super::allocate_memory;
use crate::emu;

pub fn _set_error_mode(emu: &mut emu::Emu) {
    let mode = emu.regs().rcx as u32;
    log_red!(emu, "msvcrt!_set_error_mode mode:{}", mode);
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, mode);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // previous mode (_OUT_TO_DEFAULT)
}
