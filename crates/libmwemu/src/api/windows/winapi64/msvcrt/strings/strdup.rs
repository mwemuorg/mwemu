use crate::emu;
use crate::winapi::winapi64::msvcrt::allocate_memory;

pub fn strdup(emu: &mut emu::Emu, api: &str) {
    let src_ptr = emu.regs().rcx;
    let s = emu.maps.read_string(src_ptr);
    let len = s.len() as u64 + 1;
    let dst = allocate_memory(emu, len).unwrap_or(0);
    if dst != 0 {
        emu.maps.write_string(dst, &s);
    }
    log_red!(emu, "msvcrt!{} '{}' => 0x{:x}", api, s, dst);
    emu.set_last_error(0);
    emu.regs_mut().rax = dst;
}
