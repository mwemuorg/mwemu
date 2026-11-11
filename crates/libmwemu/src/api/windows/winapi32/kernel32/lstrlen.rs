use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn lstrlen(emu: &mut emu::Emu) {
    let s_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    emu.stack_pop32(false);
    let s = emu.maps.read_string(s_ptr);
    let len = s.len() as u64;

    log_red!(emu, "kernel32!lstrlen '{}' ={}", s, len);

    emu.regs_mut().rax = len;
}

pub fn lstrlenW(emu: &mut emu::Emu) {
    let s = emu.maps.read_wide_string(arg(emu, 0) as u64);
    let len = s.chars().count() as u64;
    log_red!(emu, "kernel32!lstrlenW '{}' ={}", s, len);
    emu.regs_mut().rax = len;
    stdcall_ret(emu, 1);
}
