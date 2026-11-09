use super::allocate_memory;
use crate::emu;

pub fn _wcmdln(emu: &mut emu::Emu, api: &str) {
    let addr = allocate_memory(emu, 64).unwrap_or(0);
    if addr != 0 {
        emu.maps.write_wide_string(addr, "mwemu.exe");
    }
    log_red!(emu, "msvcrt!{} => 0x{:x}", api, addr);
    emu.set_last_error(0);
    emu.regs_mut().rax = addr;
}
