use super::find_window;
use crate::emu;

pub fn FindWindowExA(emu: &mut emu::Emu) {
    find_window_ex(emu, "FindWindowExA", false);
}

pub fn FindWindowExW(emu: &mut emu::Emu) {
    find_window_ex(emu, "FindWindowExW", true);
}

fn find_window_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let (parent, after) = (emu.regs().rcx, emu.regs().rdx);
    let (class_ptr, window_ptr) = (emu.regs().r8, emu.regs().r9);
    find_window(emu, api, parent, after, class_ptr, window_ptr, wide);
}
