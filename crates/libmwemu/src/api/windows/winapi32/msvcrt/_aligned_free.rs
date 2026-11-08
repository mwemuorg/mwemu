use super::release;
use crate::emu;

pub fn _aligned_free(emu: &mut emu::Emu, addr: u32) {
    let addr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "msvcrt!_aligned_free 0x{:x}", addr);
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    release(emu, addr);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
