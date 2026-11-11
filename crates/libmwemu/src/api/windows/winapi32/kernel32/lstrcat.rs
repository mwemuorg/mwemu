use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn lstrcat(emu: &mut emu::Emu) {
    let str1_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let str2_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    let mut str1 = emu.maps.read_string(str1_ptr);
    let str2 = emu.maps.read_string(str2_ptr);

    log_red!(emu, "kernel32!lstrcat '{}'+'{}'", str1, str2);

    str1.push_str(&str2);

    emu.maps.write_string(str1_ptr, &str1);

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}

pub fn lstrcatW(emu: &mut emu::Emu) {
    let str1_ptr = arg(emu, 0) as u64;
    let str2_ptr = arg(emu, 1) as u64;
    let mut str1 = emu.maps.read_wide_string(str1_ptr);
    let str2 = emu.maps.read_wide_string(str2_ptr);
    log_red!(emu, "kernel32!lstrcatW '{}'+'{}'", str1, str2);
    str1.push_str(&str2);
    emu.maps.write_wide_string(str1_ptr, &str1);
    emu.regs_mut().rax = str1_ptr;
    stdcall_ret(emu, 2);
}
