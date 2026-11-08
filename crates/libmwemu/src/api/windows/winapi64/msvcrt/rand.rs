use super::allocate_memory;
use crate::emu;

pub fn rand(emu: &mut emu::Emu) {
    let seed = allocate_memory(emu, 4).unwrap_or(0);
    let val = if seed != 0 {
        let s = emu.maps.read_dword(seed).unwrap_or(12345);
        let next = s.wrapping_mul(214013).wrapping_add(2531011);
        emu.maps.write_dword(seed, next);
        ((next >> 16) & 0x7fff) as u64
    } else {
        42
    };
    log_red!(emu, "msvcrt!rand => {}", val);
    emu.set_last_error(0);
    emu.regs_mut().rax = val;
}
