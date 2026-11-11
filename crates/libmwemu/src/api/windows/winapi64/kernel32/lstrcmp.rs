use crate::emu;

pub fn lstrcmpA_api(emu: &mut emu::Emu) {
    let s1 = emu.regs().rcx;
    let s2 = emu.regs().rdx;
    let str1 = emu.maps.read_string(s1);
    let str2 = emu.maps.read_string(s2);
    let r = str1.cmp(&str2) as i32;
    log_red!(emu, "kernel32!lstrcmpA '{}' '{}' => {}", str1, str2, r);
    emu.regs_mut().rax = r as u64;
}

pub fn lstrcmpW_api(emu: &mut emu::Emu) {
    let s1 = emu.regs().rcx;
    let s2 = emu.regs().rdx;
    let str1 = emu.maps.read_wide_string(s1);
    let str2 = emu.maps.read_wide_string(s2);
    let r = str1.cmp(&str2) as i32;
    log_red!(emu, "kernel32!lstrcmpW '{}' '{}' => {}", str1, str2, r);
    emu.regs_mut().rax = r as u64;
}
