use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn lstrcpyW(emu: &mut emu::Emu) {
    let dst = arg(emu, 0) as u64;
    let src = arg(emu, 1) as u64;
    let s = emu.maps.read_wide_string(src);
    emu.maps.write_wide_string(dst, &s);
    log_red!(emu, "kernel32!lstrcpyW 0x{:x} 0x{:x} '{}'", dst, src, s);
    emu.regs_mut().rax = dst;
    stdcall_ret(emu, 2);
}
