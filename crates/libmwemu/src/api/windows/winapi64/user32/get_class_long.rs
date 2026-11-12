use crate::emu;
use crate::winapi::common::strings;

pub fn GetClassLongA(emu: &mut emu::Emu) {
    get_class_long(emu, "GetClassLongA", false);
}

fn get_class_long(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!{api} '{}' p1:0x{:x}", s, p1);
}

pub fn GetClassLongW(emu: &mut emu::Emu) {
    get_class_long(emu, "GetClassLongW", true);
}
