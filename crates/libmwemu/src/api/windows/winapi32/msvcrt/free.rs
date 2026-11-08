use super::release;
use crate::emu;

pub(super) fn free(emu: &mut emu::Emu) {
    let addr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "msvcrt!free 0x{:x}", addr);

    if emu.cfg.memory_guard {
        emu.kernel_free(addr as u64, "free");
    } else {
        release(emu, addr as u64);
    }
}
