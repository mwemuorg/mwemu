use super::allocate_memory;
use crate::emu;

pub fn calloc(emu: &mut emu::Emu) {
    let nmemb = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let total = nmemb.saturating_mul(size);
    if total == 0 {
        emu.regs_mut().rax = 0;
    } else {
        let base = allocate_memory(emu, total).unwrap_or(0);
        if base != 0 {
            emu.maps.memset(base, 0, total as usize);
        }
        log_red!(emu, "msvcrt!calloc {}*{} =0x{:x}", nmemb, size, base);
        let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
        if first_param != 0 && emu.maps.is_mapped(first_param) {
            let _ = emu.maps.read_dword(first_param).unwrap_or(0);
        }
        emu.regs_mut().rax = base;
    }
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
