use crate::emu;
use crate::winapi::winapi32::msvcrt::allocate_memory;

pub fn _iob(emu: &mut emu::Emu) {
    let iob_addr = allocate_memory(emu, 96).unwrap_or(0);
    log_red!(emu, "msvcrt!_iob => 0x{:x}", iob_addr);
    emu.set_last_error(0);
    emu.regs_mut().rax = iob_addr;
}
