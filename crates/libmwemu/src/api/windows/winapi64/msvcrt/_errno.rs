use super::allocate_memory;
use crate::emu;

pub fn _errno(emu: &mut emu::Emu) {
    let errno_addr = match emu.maps.get_map_by_name("msvcrt_errno") {
        Some(m) => m.get_base(),
        None => {
            let addr = allocate_memory(emu, 8).unwrap_or(0);
            if addr != 0 {
                emu.maps.write_dword(addr, 0);
            }
            addr
        }
    };
    log_red!(emu, "msvcrt!_errno => 0x{:x}", errno_addr);
    emu.set_last_error(0);
    emu.regs_mut().rax = errno_addr;
}
