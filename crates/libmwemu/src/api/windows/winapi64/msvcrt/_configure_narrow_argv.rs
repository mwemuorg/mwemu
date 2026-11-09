use super::allocate_memory;
use crate::emu;

pub fn _configure_narrow_argv(emu: &mut emu::Emu) {
    let mode = emu.regs().rcx;
    log_red!(emu, "msvcrt!_configure_narrow_argv mode:{}", mode);
    let slot = allocate_memory(emu, 8).unwrap_or(0);
    if slot != 0 {
        emu.maps.write_qword(slot, mode);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // success
}
