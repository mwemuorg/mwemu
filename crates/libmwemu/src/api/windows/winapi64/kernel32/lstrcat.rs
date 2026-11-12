use crate::emu;
use crate::winapi::common::strings;

pub fn lstrcatA(emu: &mut emu::Emu) {
    lstrcat(emu, "lstrcatA", false);
}

fn lstrcat(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str1_ptr = emu.regs().rcx;
    let str2_ptr = emu.regs().rdx;

    let mut str1 = strings::read_text(emu, str1_ptr, wide);
    let str2 = strings::read_text(emu, str2_ptr, wide);

    log_red!(emu, "kernel32!{api} '{}'+'{}'", str1, str2);

    str1.push_str(&str2);
    strings::write_string(emu, str1_ptr, &str1, wide);

    emu.regs_mut().rax = 1;
}

pub fn lstrcatW(emu: &mut emu::Emu) {
    lstrcat(emu, "lstrcatW", true);
}
