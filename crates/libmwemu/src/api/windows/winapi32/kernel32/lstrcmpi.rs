use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants::errors::*;

pub fn lstrcmpiA(emu: &mut emu::Emu) {
    let lpString1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpString2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let lpString1 = emu.maps.read_string(lpString1 as u64);
    let lpString2 = emu.maps.read_string(lpString2 as u64);

    log_red!(emu, "kernel32!lstrcmpiA {} {}", lpString1, lpString2);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn lstrcmpiW(emu: &mut emu::Emu) {
    let s1 = emu.maps.read_wide_string(arg(emu, 0) as u64);
    let s2 = emu.maps.read_wide_string(arg(emu, 1) as u64);
    let result = match s1.to_lowercase().cmp(&s2.to_lowercase()) {
        std::cmp::Ordering::Less => -1i32,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };
    log_red!(emu, "kernel32!lstrcmpiW '{}' '{}' ={}", s1, s2, result);
    emu.regs_mut().rax = result as u32 as u64;
    stdcall_ret(emu, 2);
}
