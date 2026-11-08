use super::allocate_memory;
use crate::emu;

pub fn _amsg_exit(emu: &mut emu::Emu) {
    let code = emu.regs().rcx as u32;
    log_red!(emu, "msvcrt!_amsg_exit code:{}", code);
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, code);
    }
    emu.stop();
}
