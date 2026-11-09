use super::allocate_memory;
use crate::emu;

pub fn __p__environ(emu: &mut emu::Emu) {
    let env_addr = allocate_memory(emu, 8).unwrap_or(0);
    if env_addr != 0 {
        emu.maps.write_dword(env_addr, 0); // NULL-terminated environ
    }
    log_red!(emu, "msvcrt!__p__environ => 0x{:x}", env_addr);
    emu.regs_mut().rax = env_addr;
}
