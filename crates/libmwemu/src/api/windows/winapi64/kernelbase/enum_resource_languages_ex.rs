use crate::emu;
use crate::winapi::common::strings;

pub fn EnumResourceLanguagesExA(emu: &mut emu::Emu) {
    enum_resource_languages_ex(emu, "EnumResourceLanguagesExA", false);
}

fn enum_resource_languages_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let p3 = emu.regs().r9;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernelbase!{api} '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn EnumResourceLanguagesExW(emu: &mut emu::Emu) {
    enum_resource_languages_ex(emu, "EnumResourceLanguagesExW", true);
}
