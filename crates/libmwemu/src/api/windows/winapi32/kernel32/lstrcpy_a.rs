use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn lstrcpyA(emu: &mut emu::Emu) {
    let dst = arg(emu, 0) as u64;
    let src = arg(emu, 1) as u64;

    let s = emu.maps.read_string(src);

    log_red!(emu, "kernel32!lstrcpyA '{}' dst:0x{:x}", s, dst);

    if dst != 0 && !s.is_empty() {
        emu.maps.write_string(dst, &s);
        emu.regs_mut().rax = dst;
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 2);
}
