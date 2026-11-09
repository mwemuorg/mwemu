use super::allocate_memory;
use crate::emu;

pub(super) fn _errno(emu: &mut emu::Emu) {
    let addr = allocate_memory(emu, 4).unwrap_or(0);
    if addr != 0 {
        emu.maps.write_dword(addr, 0);
    }
    log_red!(emu, "msvcrt!_errno => 0x{:x}", addr);
    emu.regs_mut().rax = addr;
}
