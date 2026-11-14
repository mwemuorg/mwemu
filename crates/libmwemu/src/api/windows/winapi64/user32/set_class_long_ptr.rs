use crate::emu;
use crate::winapi::common::strings;

pub fn SetClassLongPtrA(emu: &mut emu::Emu) {
    set_class_long_ptr(emu, "SetClassLongPtrA", false);
}

fn set_class_long_ptr(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "user32!{api} '{}' p1:0x{:x} p2:0x{:x}", s, p1, p2);
}

pub fn SetClassLongPtrW(emu: &mut emu::Emu) {
    set_class_long_ptr(emu, "SetClassLongPtrW", true);
}
