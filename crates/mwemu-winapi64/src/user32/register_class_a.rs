use super::register_class;
use crate::emu;

pub(super) fn RegisterClassA(emu: &mut emu::Emu) {
    let lpwndclass = emu.regs().rcx;
    let atom = 0xc000u64 + (lpwndclass & 0xfff);
    log_red!(
        emu,
        "user32!RegisterClassA wndclass:0x{:x} => atom:0x{:x}",
        lpwndclass,
        atom
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = atom;
    log_red!(emu, "** {} user32!RegisterClassA {}", emu.pos, lpwndclass);
    register_class(emu, lpwndclass, false);
}
