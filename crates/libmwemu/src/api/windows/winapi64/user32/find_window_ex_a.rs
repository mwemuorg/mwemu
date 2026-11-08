use super::find_window;
use crate::emu;

pub fn FindWindowExA(emu: &mut emu::Emu) {
    let (parent, after) = (emu.regs().rcx, emu.regs().rdx);
    let (class_ptr, window_ptr) = (emu.regs().r8, emu.regs().r9);
    find_window(
        emu,
        "FindWindowExA",
        parent,
        after,
        class_ptr,
        window_ptr,
        false,
    );
}
