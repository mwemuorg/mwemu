use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn lstrcpyn(emu: &mut emu::Emu) {
    let dst = arg(emu, 0) as u64;
    let src = arg(emu, 1) as u64;
    let len = arg(emu, 2) as usize;
    let mut s = emu.maps.read_string(src);
    if len > 0 && s.len() >= len {
        s = s.chars().take(len - 1).collect();
    }
    emu.maps.write_string(dst, &s);
    log_red!(emu, "kernel32!lstrcpyn '{}' max: {}", s, len);
    emu.regs_mut().rax = dst;
    stdcall_ret(emu, 3);
}
