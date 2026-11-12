use super::register_class;
use crate::emu;

pub(super) fn RegisterClassExA(emu: &mut emu::Emu) {
    register_class_ex(emu, "RegisterClassExA", false);
}

pub(super) fn RegisterClassExW(emu: &mut emu::Emu) {
    register_class_ex(emu, "RegisterClassExW", true);
}

fn register_class_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lpwndclass = emu.regs().rcx;
    log_red!(emu, "user32!{api} 0x{:x}", lpwndclass);
    register_class(emu, lpwndclass, wide);
}
