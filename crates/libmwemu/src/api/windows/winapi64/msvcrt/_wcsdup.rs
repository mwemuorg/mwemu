use super::allocate_memory;
use crate::emu;

pub fn _wcsdup(emu: &mut emu::Emu) {
    let src_ptr = emu.regs().rcx;
    let s = emu.maps.read_wide_string(src_ptr);
    let len = (s.len() as u64 + 1) * 2;
    let dst = allocate_memory(emu, len).unwrap_or(0);
    if dst != 0 {
        emu.maps.write_wide_string(dst, &s);
    }
    log_red!(emu, "msvcrt!_wcsdup '{}' => 0x{:x}", s, dst);
    emu.set_last_error(0);
    emu.regs_mut().rax = dst;
}
