use super::allocate_memory;
use crate::emu;

pub fn _initialize_narrow_environment(emu: &mut emu::Emu) {
    log_red!(emu, "msvcrt!_initialize_narrow_environment");
    let env_addr = allocate_memory(emu, 16).unwrap_or(0);
    if env_addr != 0 {
        emu.maps.write_qword(env_addr, 0); // empty environment
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // success
}
