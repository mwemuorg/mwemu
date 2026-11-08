use super::allocate_memory;
use crate::emu;

pub fn _onexit(emu: &mut emu::Emu) {
    let fptr = emu.regs().rcx;
    log_red!(emu, "msvcrt!_onexit 0x{:x}", fptr);
    let slot = allocate_memory(emu, 8).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_qword(slot, fptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = fptr;
}
