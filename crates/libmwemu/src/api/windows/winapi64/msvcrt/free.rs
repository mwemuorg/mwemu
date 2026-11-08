use super::release;
use crate::emu;

pub fn free(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    log_red!(emu, "msvcrt!free 0x{:x}", addr);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    if emu.cfg.memory_guard {
        emu.kernel_free(addr, "free");
    } else {
        release(emu, addr);
    }
}
