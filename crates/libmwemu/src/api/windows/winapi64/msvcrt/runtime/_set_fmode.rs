use crate::emu;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn _set_fmode(emu: &mut emu::Emu) {
    let mode = emu.regs().rcx as u32;
    log_red!(emu, "msvcrt!_set_fmode mode:0x{:x}", mode);
    let slot = allocate_memory(emu, 4).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_dword(slot, mode);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // success
}
