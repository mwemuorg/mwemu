use super::allocate_memory;
use crate::emu;

pub fn __setlc_active(emu: &mut emu::Emu, api: &str) {
    let addr = allocate_memory(emu, 4).unwrap_or(0);
    if addr != 0 {
        emu.maps.write_dword(addr, 0);
    }
    log_red!(emu, "msvcrt!{} => 0x{:x}", api, addr);
    emu.set_last_error(0);
    emu.regs_mut().rax = addr;
}
