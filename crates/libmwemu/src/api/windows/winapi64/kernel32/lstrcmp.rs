use crate::emu;
use crate::winapi::common::strings;

pub fn lstrcmpA(emu: &mut emu::Emu) {
    lstrcmp(emu, "lstrcmpA", false);
}

fn lstrcmp(emu: &mut emu::Emu, api: &str, wide: bool) {
    let s1 = emu.regs().rcx;
    let s2 = emu.regs().rdx;
    let str1 = strings::read_text(emu, s1, wide);
    let str2 = strings::read_text(emu, s2, wide);
    let r = str1.cmp(&str2) as i32;
    log_red!(emu, "kernel32!{api} '{}' '{}' => {}", str1, str2, r);
    emu.regs_mut().rax = r as u64;
}

pub fn lstrcmpW(emu: &mut emu::Emu) {
    lstrcmp(emu, "lstrcmpW", true);
}
