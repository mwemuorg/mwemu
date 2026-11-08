use super::allocate_memory;
use crate::emu;

pub fn _aligned_malloc(emu: &mut emu::Emu) {
    let size = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let _alignment = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(16);
    let base = allocate_memory(emu, size).unwrap_or(0);
    log_red!(emu, "msvcrt!_aligned_malloc {} =0x{:x}", size, base);
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = base;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
