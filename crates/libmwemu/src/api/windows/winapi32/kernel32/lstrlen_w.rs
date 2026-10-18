use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn lstrlenW(emu: &mut emu::Emu) {
    let s = emu.maps.read_wide_string(arg(emu, 0) as u64);
    let len = s.chars().count() as u64;
    log_red!(emu, "kernel32!lstrlenW '{}' ={}", s, len);
    emu.regs_mut().rax = len;
    stdcall_ret(emu, 1);
}
