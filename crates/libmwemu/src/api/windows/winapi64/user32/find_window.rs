use super::find_window;
use crate::emu;

pub fn FindWindowA(emu: &mut emu::Emu) {
    find_window_text(emu, "FindWindowA", false);
}

pub fn FindWindowW(emu: &mut emu::Emu) {
    find_window_text(emu, "FindWindowW", true);
}

fn find_window_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let class_ptr = emu.regs().rcx;
    let window_ptr = emu.regs().rdx;
    find_window(emu, api, 0, 0, class_ptr, window_ptr, wide);
}
