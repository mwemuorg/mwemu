use super::allocate_memory;
use crate::emu;

pub fn srand(emu: &mut emu::Emu) {
    let seed = emu.regs().rcx as u32;
    log_red!(emu, "msvcrt!srand seed:{}", seed);
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, seed);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
