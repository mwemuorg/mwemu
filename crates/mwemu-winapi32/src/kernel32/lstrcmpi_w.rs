use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

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
