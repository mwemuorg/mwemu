use super::allocate_memory;
use crate::emu;

pub(super) fn crt_iob(emu: &mut emu::Emu) {
    let iob_addr = allocate_memory(emu, 96).unwrap_or(0);
    log_red!(emu, "wincrt!_iob => 0x{:x}", iob_addr);
    emu.regs_mut().rax = iob_addr;
}
