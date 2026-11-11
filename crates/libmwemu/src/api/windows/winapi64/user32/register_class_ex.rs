use super::register_class;
use crate::emu;

pub(super) fn RegisterClassExA(emu: &mut emu::Emu) {
    let lpwndclass = emu.regs().rcx;
    log_red!(emu, "user32!RegisterClassExA 0x{:x}", lpwndclass);
    register_class(emu, lpwndclass, false);
}

pub(super) fn RegisterClassExW(emu: &mut emu::Emu) {
    let lpwndclass = emu.regs().rcx;
    log_red!(emu, "user32!RegisterClassExW 0x{:x}", lpwndclass);
    register_class(emu, lpwndclass, true);
}
