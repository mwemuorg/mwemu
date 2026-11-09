use super::register_class;
use crate::emu;

pub(super) fn RegisterClassW(emu: &mut emu::Emu) {
    let lpwndclass = emu.regs().rcx;
    let atom = 0xc000u64 + (lpwndclass & 0xfff);
    log_red!(
        emu,
        "user32!RegisterClassW wndclass:0x{:x} => atom:0x{:x}",
        lpwndclass,
        atom
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = atom;
    log_red!(emu, "** {} user32!RegisterClassW {}", emu.pos, lpwndclass);
    register_class(emu, lpwndclass, true);
}
