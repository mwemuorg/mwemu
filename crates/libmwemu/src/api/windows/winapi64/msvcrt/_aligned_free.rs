use super::release;
use crate::emu;

pub fn _aligned_free(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    log_red!(emu, "msvcrt!_aligned_free 0x{:x}", addr);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    release(emu, addr);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
