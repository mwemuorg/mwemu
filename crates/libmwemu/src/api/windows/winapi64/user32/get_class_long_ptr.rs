use crate::emu;
use crate::winapi::common::strings;

pub fn GetClassLongPtrA(emu: &mut emu::Emu) {
    get_class_long_ptr(emu, "GetClassLongPtrA", false);
}

fn get_class_long_ptr(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!{api} '{}' p1:0x{:x}", s, p1);
}

pub fn GetClassLongPtrW(emu: &mut emu::Emu) {
    get_class_long_ptr(emu, "GetClassLongPtrW", true);
}
